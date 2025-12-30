// Generated macro for lint_args (function)
macro_rules! Depcrate_core_build_steps_clippylint_args {
() => {
// Module: crate::core::build_steps::clippy
// Provides: {"lint_args"}
// Dependencies: {}
fn lint_args (builder : & Builder < '_ > , config : & LintConfig , ignored_rules : & [& str]) -> Vec < String > { fn strings < 'a > (arr : & 'a [& str]) -> impl Iterator < Item = String > + 'a { arr . iter () . copied () . map (String :: from) } let Subcommand :: Clippy { fix , allow_dirty , allow_staged , .. } = & builder . config . cmd else { unreachable ! ("clippy::lint_args can only be called from `clippy` subcommands.") ; } ; let mut args = vec ! [] ; if * fix { # [rustfmt :: skip] args . extend (strings (& ["--fix" , "-Zunstable-options" , "--lib" , "--bins" , "--examples" ,])) ; if * allow_dirty { args . push ("--allow-dirty" . to_owned ()) ; } if * allow_staged { args . push ("--allow-staged" . to_owned ()) ; } } args . extend (strings (& ["--"])) ; if config . deny . is_empty () && config . forbid . is_empty () { args . extend (strings (& ["--cap-lints" , "warn"])) ; } let all_args = std :: env :: args () . collect :: < Vec < _ > > () ; args . extend (get_clippy_rules_in_order (& all_args , config)) ; args . extend (ignored_rules . iter () . map (| lint | format ! ("-Aclippy::{lint}"))) ; args . extend (builder . config . free_args . clone ()) ; args }
};
}
