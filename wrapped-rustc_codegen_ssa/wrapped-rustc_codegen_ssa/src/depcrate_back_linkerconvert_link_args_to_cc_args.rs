// Generated macro for convert_link_args_to_cc_args (function)
macro_rules! Depcrate_back_linkerconvert_link_args_to_cc_args {
() => {
// Module: crate::back::linker
// Provides: {"convert_link_args_to_cc_args"}
// Dependencies: {}
# [doc = " Add underlying linker arguments to C compiler command, by wrapping them in"] # [doc = " `-Wl` or `-Xlinker`."] fn convert_link_args_to_cc_args (cmd : & mut Command , args : impl IntoIterator < Item : AsRef < OsStr > >) { let mut combined_arg = OsString :: from ("-Wl") ; for arg in args { if arg . as_ref () . as_encoded_bytes () . contains (& b',') { if combined_arg != OsStr :: new ("-Wl") { cmd . arg (combined_arg) ; combined_arg = OsString :: from ("-Wl") ; } cmd . arg ("-Xlinker") ; cmd . arg (arg) ; } else { combined_arg . push (",") ; combined_arg . push (arg) ; } } if combined_arg != OsStr :: new ("-Wl") { cmd . arg (combined_arg) ; } }
};
}
