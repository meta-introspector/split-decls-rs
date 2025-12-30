// Generated macro for describe_codegen_flags (function)
macro_rules! Depcratedescribe_codegen_flags {
() => {
// Module: crate
// Provides: {"describe_codegen_flags"}
// Dependencies: {}
fn describe_codegen_flags () { safe_println ! ("\nAvailable codegen options:\n") ; print_flag_list ("-C" , config :: CG_OPTIONS) ; }
};
}
