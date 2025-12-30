// Generated macro for check_one_checkfile (function)
macro_rules! Depcratecheck_one_checkfile {
() => {
// Module: crate
// Provides: {"check_one_checkfile"}
// Dependencies: {}
fn check_one_checkfile (path : & Path , args : & Args , files_failed : & mut u64) -> anyhow :: Result < () > { let mut file ; let stdin ; let mut stdin_lock ; let mut bufreader : io :: BufReader < & mut dyn Read > ; if path == Path :: new ("-") { stdin = io :: stdin () ; stdin_lock = stdin . lock () ; bufreader = io :: BufReader :: new (& mut stdin_lock) ; } else { file = File :: open (path) ? ; bufreader = io :: BufReader :: new (& mut file) ; } let mut line = String :: new () ; loop { line . clear () ; let n = bufreader . read_line (& mut line) ? ; if n == 0 { return Ok (()) ; } let success = check_one_line (& line , args) ; if ! success { * files_failed = files_failed . saturating_add (1) ; } } }
};
}
