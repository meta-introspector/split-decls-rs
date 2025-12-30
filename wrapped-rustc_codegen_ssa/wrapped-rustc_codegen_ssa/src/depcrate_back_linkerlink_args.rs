// Generated macro for link_args (function)
macro_rules! Depcrate_back_linkerlink_args {
() => {
// Module: crate::back::linker
// Provides: {"link_args"}
// Dependencies: {}
# [doc = " Arguments for the underlying linker."] # [doc = " Add options to pass them through cc wrapper if `Linker` is a cc wrapper."] fn link_args < L : Linker + ? Sized > (l : & mut L , args : impl IntoIterator < Item : AsRef < OsStr > >) -> & mut L { if ! l . is_cc () { verbatim_args (l , args) ; } else { convert_link_args_to_cc_args (l . cmd () , args) ; } l }
};
}
