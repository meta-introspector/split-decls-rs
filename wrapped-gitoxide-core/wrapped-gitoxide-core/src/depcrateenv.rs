// Generated macro for env (function)
macro_rules! Depcrateenv {
() => {
// Module: crate
// Provides: {"env"}
// Dependencies: {}
pub fn env (mut out : impl std :: io :: Write , format : OutputFormat) -> anyhow :: Result < () > { if format != OutputFormat :: Human { bail ! ("JSON output isn't supported") ; } let width = 15 ; writeln ! (out , "{field:>width$}: {}" , std :: path :: Path :: new (gix :: path :: env :: shell ()) . display () , field = "shell" ,) ? ; writeln ! (out , "{field:>width$}: {:?}" , gix :: path :: env :: installation_config_prefix () , field = "config prefix" ,) ? ; writeln ! (out , "{field:>width$}: {:?}" , gix :: path :: env :: installation_config () , field = "config" ,) ? ; writeln ! (out , "{field:>width$}: {}" , gix :: path :: env :: exe_invocation () . display () , field = "git exe" ,) ? ; writeln ! (out , "{field:>width$}: {:?}" , gix :: path :: env :: system_prefix () , field = "system prefix" ,) ? ; writeln ! (out , "{field:>width$}: {:?}" , gix :: path :: env :: core_dir () , field = "core dir" ,) ? ; Ok (()) }
};
}
