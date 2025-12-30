// Generated macro for discover (function)
macro_rules! Depcrate_discoverdiscover {
() => {
// Module: crate::discover
// Provides: {"discover"}
// Dependencies: {}
pub fn discover (repo : & Path , mut out : impl std :: io :: Write) -> anyhow :: Result < () > { let mut has_err = false ; writeln ! (out , "open (strict) {}:" , repo . display ()) ? ; has_err |= print_result (& mut out , gix :: open_opts (repo , gix :: open :: Options :: default () . strict_config (true)) ,) ? ; if has_err { writeln ! (out , "open (lenient) {}:" , repo . display ()) ? ; has_err |= print_result (& mut out , gix :: open_opts (repo , gix :: open :: Options :: default () . strict_config (false)) ,) ? ; } writeln ! (out) ? ; writeln ! (out , "discover from {}:" , repo . display ()) ? ; has_err |= print_result (& mut out , gix :: discover (repo)) ? ; writeln ! (out) ? ; writeln ! (out , "discover (plumbing) from {}:" , repo . display ()) ? ; has_err |= print_result (& mut out , gix :: discover :: upwards (repo)) ? ; if has_err { writeln ! (out) ? ; anyhow :: bail ! ("At least one operation failed") } Ok (()) }
};
}
