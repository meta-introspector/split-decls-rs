// Generated macro for cc_args (function)
macro_rules! Depcrate_back_linkercc_args {
() => {
// Module: crate::back::linker
// Provides: {"cc_args"}
// Dependencies: {}
# [doc = " Arguments for the cc wrapper specifically."] # [doc = " Check that it's indeed a cc wrapper and pass verbatim."] fn cc_args < L : Linker + ? Sized > (l : & mut L , args : impl IntoIterator < Item : AsRef < OsStr > >) -> & mut L { assert ! (l . is_cc ()) ; verbatim_args (l , args) }
};
}
