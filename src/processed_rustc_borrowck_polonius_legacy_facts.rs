// SRC: ../rust/compiler/rustc_borrowck/src/polonius/legacy/facts.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use std::error::Error;
use std::fmt::Debug;
use std::fs::{self, File};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
use std::io::Write;
use std::path::Path;

use polonius_engine::{AllFacts, Atom, Output};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use rustc_macros::extension;
use crate::rustc_complete::mir::Local;
use crate::rustc_complete::ty::{RegionVid, TyCtxt};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use crate::rustc_mir_dataflow::move_paths::MovePathIndex;

use super::{LocationIndex, PoloniusLocationTable};
/* AST_META: AST_ID=5 | TYPE=STRUCT | NAME=RustcFacts; | COMPLEXITY=4 | LINES=13 */
use crate::BorrowIndex;

#[derive(Copy, Clone, Debug)]
pub struct RustcFacts;

pub type PoloniusOutput = Output<RustcFacts>;

crate::rustc_index::newtype_index! {
    /// A (kinda) newtype of `RegionVid` so we can implement `Atom` on it.
    #[orderable]
    #[debug_format = "'?{}"]
    pub struct PoloniusRegionVid {}
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=index | COMPLEXITY=5 | LINES=6 */

impl polonius_engine::Atom for PoloniusRegionVid {
    fn index(self) -> usize {
        self.as_usize()
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=5 */
impl From<RegionVid> for PoloniusRegionVid {
    fn from(value: RegionVid) -> Self {
        Self::from_usize(value.as_usize())
    }
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=5 */
impl From<PoloniusRegionVid> for RegionVid {
    fn from(value: PoloniusRegionVid) -> Self {
        Self::from_usize(value.as_usize())
    }
}
/* AST_META: AST_ID=9 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=8 */

impl polonius_engine::FactTypes for RustcFacts {
    type Origin = PoloniusRegionVid;
    type Loan = BorrowIndex;
    type Point = LocationIndex;
    type Variable = Local;
    type Path = MovePathIndex;
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=enabled | COMPLEXITY=19 | LINES=57 */

pub type PoloniusFacts = AllFacts<RustcFacts>;

#[extension(pub(crate) trait PoloniusFactsExt)]
impl PoloniusFacts {
    /// Returns `true` if there is a need to gather `PoloniusFacts` given the
    /// current `-Z` flags.
    fn enabled(tcx: TyCtxt<'_>) -> bool {
        tcx.sess.opts.unstable_opts.nll_facts
            || tcx.sess.opts.unstable_opts.polonius.is_legacy_enabled()
    }

    fn write_to_dir(
        &self,
        dir: impl AsRef<Path>,
        location_table: &PoloniusLocationTable,
    ) -> Result<(), Box<dyn Error>> {
        let dir: &Path = dir.as_ref();
        fs::create_dir_all(dir)?;
        let wr = FactWriter { location_table, dir };
        macro_rules! write_facts_to_path {
            ($wr:ident . write_facts_to_path($this:ident . [
                $($field:ident,)*
            ])) => {
                $(
                    $wr.write_facts_to_path(
                        &$this.$field,
                        &format!("{}.facts", stringify!($field))
                    )?;
                )*
            }
        }
        write_facts_to_path! {
            wr.write_facts_to_path(self.[
                loan_issued_at,
                universal_region,
                cfg_edge,
                loan_killed_at,
                subset_base,
                loan_invalidated_at,
                var_used_at,
                var_defined_at,
                var_dropped_at,
                use_of_var_derefs_origin,
                drop_of_var_derefs_origin,
                child_path,
                path_is_var,
                path_assigned_at_base,
                path_moved_at_base,
                path_accessed_at_base,
                known_placeholder_subset,
                placeholder,
            ])
        }
        Ok(())
    }
}
/* AST_META: AST_ID=11 | TYPE=FUNCTION | NAME=index | COMPLEXITY=5 | LINES=6 */

impl Atom for BorrowIndex {
    fn index(self) -> usize {
        self.as_usize()
    }
}
/* AST_META: AST_ID=12 | TYPE=FUNCTION | NAME=index | COMPLEXITY=5 | LINES=6 */

impl Atom for LocationIndex {
    fn index(self) -> usize {
        self.as_usize()
    }
}
/* AST_META: AST_ID=13 | TYPE=STRUCT | NAME=FactWriter | COMPLEXITY=2 | LINES=5 */

struct FactWriter<'w> {
    location_table: &'w PoloniusLocationTable,
    dir: &'w Path,
}
/* AST_META: AST_ID=14 | TYPE=FUNCTION | NAME=write_facts_to_path | COMPLEXITY=6 | LINES=14 */

impl<'w> FactWriter<'w> {
    fn write_facts_to_path<T>(&self, rows: &[T], file_name: &str) -> Result<(), Box<dyn Error>>
    where
        T: FactRow,
    {
        let file = &self.dir.join(file_name);
        let mut file = File::create_buffered(file)?;
        for row in rows {
            row.write(&mut file, self.location_table)?;
        }
        Ok(())
    }
}
/* AST_META: AST_ID=15 | TYPE=FUNCTION | NAME=write | COMPLEXITY=2 | LINES=8 */

trait FactRow {
    fn write(
        &self,
        out: &mut dyn Write,
        location_table: &PoloniusLocationTable,
    ) -> Result<(), Box<dyn Error>>;
}
/* AST_META: AST_ID=16 | TYPE=FUNCTION | NAME=write | COMPLEXITY=5 | LINES=10 */

impl FactRow for PoloniusRegionVid {
    fn write(
        &self,
        out: &mut dyn Write,
        location_table: &PoloniusLocationTable,
    ) -> Result<(), Box<dyn Error>> {
        write_row(out, location_table, &[self])
    }
}
/* AST_META: AST_ID=17 | TYPE=FUNCTION | NAME=write | COMPLEXITY=5 | LINES=14 */

impl<A, B> FactRow for (A, B)
where
    A: FactCell,
    B: FactCell,
{
    fn write(
        &self,
        out: &mut dyn Write,
        location_table: &PoloniusLocationTable,
    ) -> Result<(), Box<dyn Error>> {
        write_row(out, location_table, &[&self.0, &self.1])
    }
}
/* AST_META: AST_ID=18 | TYPE=FUNCTION | NAME=write | COMPLEXITY=5 | LINES=15 */

impl<A, B, C> FactRow for (A, B, C)
where
    A: FactCell,
    B: FactCell,
    C: FactCell,
{
    fn write(
        &self,
        out: &mut dyn Write,
        location_table: &PoloniusLocationTable,
    ) -> Result<(), Box<dyn Error>> {
        write_row(out, location_table, &[&self.0, &self.1, &self.2])
    }
}
/* AST_META: AST_ID=19 | TYPE=FUNCTION | NAME=write_row | COMPLEXITY=11 | LINES=12 */

fn write_row(
    out: &mut dyn Write,
    location_table: &PoloniusLocationTable,
    columns: &[&dyn FactCell],
) -> Result<(), Box<dyn Error>> {
    for (index, c) in columns.iter().enumerate() {
        let tail = if index == columns.len() - 1 { "\n" } else { "\t" };
        write!(out, "{:?}{tail}", c.to_string(location_table))?;
    }
    Ok(())
}
/* AST_META: AST_ID=20 | TYPE=FUNCTION | NAME=to_string | COMPLEXITY=2 | LINES=4 */

trait FactCell {
    fn to_string(&self, location_table: &PoloniusLocationTable) -> String;
}
/* AST_META: AST_ID=21 | TYPE=FUNCTION | NAME=to_string | COMPLEXITY=6 | LINES=6 */

impl FactCell for BorrowIndex {
    fn to_string(&self, _location_table: &PoloniusLocationTable) -> String {
        format!("{self:?}")
    }
}
/* AST_META: AST_ID=22 | TYPE=FUNCTION | NAME=to_string | COMPLEXITY=6 | LINES=6 */

impl FactCell for Local {
    fn to_string(&self, _location_table: &PoloniusLocationTable) -> String {
        format!("{self:?}")
    }
}
/* AST_META: AST_ID=23 | TYPE=FUNCTION | NAME=to_string | COMPLEXITY=6 | LINES=6 */

impl FactCell for MovePathIndex {
    fn to_string(&self, _location_table: &PoloniusLocationTable) -> String {
        format!("{self:?}")
    }
}
/* AST_META: AST_ID=24 | TYPE=FUNCTION | NAME=to_string | COMPLEXITY=6 | LINES=6 */

impl FactCell for PoloniusRegionVid {
    fn to_string(&self, _location_table: &PoloniusLocationTable) -> String {
        format!("{self:?}")
    }
}
/* AST_META: AST_ID=25 | TYPE=FUNCTION | NAME=to_string | COMPLEXITY=6 | LINES=6 */

impl FactCell for RegionVid {
    fn to_string(&self, _location_table: &PoloniusLocationTable) -> String {
        format!("{self:?}")
    }
}
/* AST_META: AST_ID=26 | TYPE=FUNCTION | NAME=to_string | COMPLEXITY=6 | LINES=6 */

impl FactCell for LocationIndex {
    fn to_string(&self, location_table: &PoloniusLocationTable) -> String {
        format!("{:?}", location_table.to_rich_location(*self))
    }
}