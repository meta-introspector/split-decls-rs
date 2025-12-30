// Generated macro for new_context (function)
macro_rules! Depcratenew_context {
() => {
// Module: crate
// Provides: {"new_context"}
// Dependencies: {}
fn new_context < 'gcc , 'tcx > (tcx : TyCtxt < 'tcx >) -> Context < 'gcc > { let context = Context :: default () ; if tcx . sess . target . arch == "x86" || tcx . sess . target . arch == "x86_64" { context . add_command_line_option ("-masm=intel") ; } # [cfg (feature = "master")] { context . set_special_chars_allowed_in_func_names ("$.*") ; let version = Version :: get () ; let version = format ! ("{}.{}.{}" , version . major , version . minor , version . patch) ; context . set_output_ident (& format ! ("rustc version {} with libgccjit {}" , rustc_interface :: util :: rustc_version_str () . unwrap_or ("unknown version") , version ,)) ; } context . add_command_line_option ("-fno-asynchronous-unwind-tables") ; context }
};
}
