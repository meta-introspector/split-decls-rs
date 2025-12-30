// Generated macro for git_init (function)
macro_rules! Depcrategit_init {
() => {
// Module: crate
// Provides: {"git_init"}
// Dependencies: {}
fn git_init () { if ! Path :: new ("linux/.git") . exists () { assert ! (Command :: new ("git") . arg ("clone") . arg ("https://github.com/torvalds/linux.git") . arg ("--filter=tree:0") . arg ("--no-checkout") . status () . unwrap () . success ()) ; } assert ! (Command :: new ("git") . arg ("sparse-checkout") . arg ("init") . current_dir ("linux") . status () . unwrap () . success ()) ; fs :: write ("linux/.git/info/sparse-checkout" , "/*
!/*/
/include/
/arch/
/scripts/
/tools/" ,) . unwrap () ; }
};
}
