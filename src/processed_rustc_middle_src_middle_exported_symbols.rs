// SRC: ../rust/compiler/rustc_middle/src/middle/exported_symbols.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::def_id::{DefId, LOCAL_CRATE};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use rustc_macros::{Decodable, Encodable, HashStable, TyDecodable, TyEncodable};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

use crate::ty::{self, GenericArgsRef, Ty, TyCtxt};
/* AST_META: AST_ID=4 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=11 */

/// The SymbolExportLevel of a symbols specifies from which kinds of crates
/// the symbol will be exported. `C` symbols will be exported from any
/// kind of crate, including cdylibs which export very few things.
/// `Rust` will only be exported if the crate produced is a Rust
/// dylib.
#[derive(Eq, PartialEq, Debug, Copy, Clone, TyEncodable, TyDecodable, HashStable)]
pub enum SymbolExportLevel {
    C,
    Rust,
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=is_below_threshold | COMPLEXITY=3 | LINES=7 */

impl SymbolExportLevel {
    pub fn is_below_threshold(self, threshold: SymbolExportLevel) -> bool {
        threshold == SymbolExportLevel::Rust // export everything from Rust dylibs
          || self == SymbolExportLevel::C
    }
}
/* AST_META: AST_ID=6 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */

/// Kind of exported symbols.
#[derive(Eq, PartialEq, Debug, Copy, Clone, Encodable, Decodable, HashStable, Hash)]
pub enum SymbolExportKind {
    Text,
    Data,
    Tls,
}
/* AST_META: AST_ID=7 | TYPE=STRUCT | NAME=SymbolExportInfo | COMPLEXITY=5 | LINES=15 */

/// The `SymbolExportInfo` of a symbols specifies symbol-related information
/// that is relevant to code generation and linking.
///
/// The difference between `used` and `rustc_std_internal_symbol` is that the
/// former is exported by LTO while the latter isn't.
#[derive(Eq, PartialEq, Debug, Copy, Clone, TyEncodable, TyDecodable, HashStable)]
pub struct SymbolExportInfo {
    pub level: SymbolExportLevel,
    pub kind: SymbolExportKind,
    /// Was the symbol marked as `#[used(compiler)]` or `#[used(linker)]`?
    pub used: bool,
    /// Was the symbol marked as `#[rustc_std_internal_symbol]`?
    pub rustc_std_internal_symbol: bool,
}
/* AST_META: AST_ID=8 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=11 */

#[derive(Eq, PartialEq, Debug, Copy, Clone, TyEncodable, TyDecodable, HashStable)]
pub enum ExportedSymbol<'tcx> {
    NonGeneric(DefId),
    Generic(DefId, GenericArgsRef<'tcx>),
    DropGlue(Ty<'tcx>),
    AsyncDropGlueCtorShim(Ty<'tcx>),
    AsyncDropGlue(DefId, Ty<'tcx>),
    ThreadLocalShim(DefId),
    NoDefId(ty::SymbolName<'tcx>),
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=symbol_name_for_local_instance | COMPLEXITY=16 | LINES=27 */

impl<'tcx> ExportedSymbol<'tcx> {
    /// This is the symbol name of an instance if it is instantiated in the
    /// local crate.
    pub fn symbol_name_for_local_instance(&self, tcx: TyCtxt<'tcx>) -> ty::SymbolName<'tcx> {
        match *self {
            ExportedSymbol::NonGeneric(def_id) => tcx.symbol_name(ty::Instance::mono(tcx, def_id)),
            ExportedSymbol::Generic(def_id, args) => {
                tcx.symbol_name(ty::Instance::new_raw(def_id, args))
            }
            ExportedSymbol::DropGlue(ty) => {
                tcx.symbol_name(ty::Instance::resolve_drop_in_place(tcx, ty))
            }
            ExportedSymbol::AsyncDropGlueCtorShim(ty) => {
                tcx.symbol_name(ty::Instance::resolve_async_drop_in_place(tcx, ty))
            }
            ExportedSymbol::AsyncDropGlue(def_id, ty) => {
                tcx.symbol_name(ty::Instance::resolve_async_drop_in_place_poll(tcx, def_id, ty))
            }
            ExportedSymbol::ThreadLocalShim(def_id) => tcx.symbol_name(ty::Instance {
                def: ty::InstanceKind::ThreadLocalShim(def_id),
                args: ty::GenericArgs::empty(),
            }),
            ExportedSymbol::NoDefId(symbol_name) => symbol_name,
        }
    }
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=metadata_symbol_name | COMPLEXITY=4 | LINES=8 */

pub fn metadata_symbol_name(tcx: TyCtxt<'_>) -> String {
    format!(
        "rust_metadata_{}_{:08x}",
        tcx.crate_name(LOCAL_CRATE),
        tcx.stable_crate_id(LOCAL_CRATE),
    )
}