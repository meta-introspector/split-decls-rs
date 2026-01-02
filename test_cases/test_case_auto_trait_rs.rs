// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/traits/auto_trait.rs
// Error: expected square brackets
// Problematic line: line 21

use crate::traits::project::ProjectAndUnifyResult;

// FIXME(twk): this is obviously not nice to duplicate like that
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum RegionTarget<'tcx> {
    Region(Region<'tcx>),
    RegionVid(RegionVid),
