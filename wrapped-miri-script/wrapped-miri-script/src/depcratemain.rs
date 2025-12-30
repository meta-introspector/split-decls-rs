// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { if ! std :: env :: var_os ("MIRI_SCRIPT_IS_GIT_SEQUENCE_EDITOR") . unwrap_or_default () . is_empty () { return Command :: squash_sequence_editor () ; } let miri_args : Vec < _ > = std :: env :: args () . take_while (| x | * x != "--") . collect () ; let remainder : Vec < _ > = std :: env :: args () . skip_while (| x | * x != "--") . collect () ; let args = Cli :: parse_from (miri_args) ; let mut command = args . command ; command . add_remainder (remainder) ? ; command . exec () ? ; Ok (()) }
};
}
