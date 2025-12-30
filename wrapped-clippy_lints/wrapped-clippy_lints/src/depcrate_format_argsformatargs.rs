// Generated macro for FormatArgs (struct)
macro_rules! Depcrate_format_argsFormatArgs {
() => {
// Module: crate::format_args
// Provides: {"FormatArgs"}
// Dependencies: {}
# [expect (clippy :: struct_field_names)] pub struct FormatArgs < 'tcx > { format_args : FormatArgsStorage , msrv : Msrv , ignore_mixed : bool , ty_msrv_map : FxHashMap < Ty < 'tcx > , Option < RustcVersion > > , has_derived_debug : FxHashMap < Ty < 'tcx > , bool > , has_pointer_format : FxHashMap < Ty < 'tcx > , bool > , }
};
}
