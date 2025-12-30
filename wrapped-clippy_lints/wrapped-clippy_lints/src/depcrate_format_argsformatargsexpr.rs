// Generated macro for FormatArgsExpr (struct)
macro_rules! Depcrate_format_argsFormatArgsExpr {
() => {
// Module: crate::format_args
// Provides: {"FormatArgsExpr"}
// Dependencies: {}
struct FormatArgsExpr < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , macro_call : & 'a MacroCall , format_args : & 'a rustc_ast :: FormatArgs , ignore_mixed : bool , msrv : & 'a Msrv , ty_msrv_map : & 'a FxHashMap < Ty < 'tcx > , Option < RustcVersion > > , has_derived_debug : & 'a mut FxHashMap < Ty < 'tcx > , bool > , has_pointer_format : & 'a mut FxHashMap < Ty < 'tcx > , bool > , }
};
}
