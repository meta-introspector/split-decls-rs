// Generated macro for git_checkout (function)
macro_rules! Depcrategit_checkout {
() => {
// Module: crate
// Provides: {"git_checkout"}
// Dependencies: {}
fn git_checkout (rev : & str) { assert ! (Command :: new ("git") . arg ("clean") . arg ("-f") . arg ("-d") . current_dir ("linux") . status () . unwrap () . success ()) ; assert ! (Command :: new ("git") . arg ("checkout") . arg (rev) . arg ("-f") . current_dir ("linux") . status () . unwrap () . success ()) ; assert ! (Command :: new ("git") . arg ("clean") . arg ("-f") . arg ("-d") . current_dir ("linux") . status () . unwrap () . success ()) ; }
};
}
