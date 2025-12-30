// Generated macro for add_relro_args (function)
macro_rules! Depcrate_back_linkadd_relro_args {
() => {
// Module: crate::back::link
// Provides: {"add_relro_args"}
// Dependencies: {}
# [doc = " Add options making relocation sections in the produced ELF files read-only"] # [doc = " and suppressing lazy binding."] fn add_relro_args (cmd : & mut dyn Linker , sess : & Session) { match sess . opts . cg . relro_level . unwrap_or (sess . target . relro_level) { RelroLevel :: Full => cmd . full_relro () , RelroLevel :: Partial => cmd . partial_relro () , RelroLevel :: Off => cmd . no_relro () , RelroLevel :: None => { } } }
};
}
