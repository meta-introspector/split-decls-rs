// Generated macro for link_or_cc_args (function)
macro_rules! Depcrate_back_linkerlink_or_cc_args {
() => {
// Module: crate::back::linker
// Provides: {"link_or_cc_args"}
// Dependencies: {}
# [doc = " Arguments supported by both underlying linker and cc wrapper, pass verbatim."] fn link_or_cc_args < L : Linker + ? Sized > (l : & mut L , args : impl IntoIterator < Item : AsRef < OsStr > > ,) -> & mut L { verbatim_args (l , args) }
};
}
