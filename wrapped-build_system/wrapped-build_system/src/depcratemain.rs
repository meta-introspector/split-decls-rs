// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { if env :: var ("RUST_BACKTRACE") . is_err () { unsafe { env :: set_var ("RUST_BACKTRACE" , "1") ; } } let command = match env :: args () . nth (1) . as_deref () { Some ("cargo") => Command :: Cargo , Some ("rustc") => Command :: Rustc , Some ("clean") => Command :: Clean , Some ("prepare") => Command :: Prepare , Some ("build") => Command :: Build , Some ("test") => Command :: Test , Some ("info") => Command :: Info , Some ("clone-gcc") => Command :: CloneGcc , Some ("abi-test") => Command :: AbiTest , Some ("fmt") => Command :: Fmt , Some ("fuzz") => Command :: Fuzz , Some ("--help") => { usage () ; process :: exit (0) ; } Some (flag) if flag . starts_with ('-') => arg_error ! ("Expected command found flag {}" , flag) , Some (command) => arg_error ! ("Unknown command {}" , command) , None => { usage () ; process :: exit (0) ; } } ; if let Err (e) = match command { Command :: Cargo => rust_tools :: run_cargo () , Command :: Rustc => rust_tools :: run_rustc () , Command :: Clean => clean :: run () , Command :: Prepare => prepare :: run () , Command :: Build => build :: run () , Command :: Test => test :: run () , Command :: Info => info :: run () , Command :: CloneGcc => clone_gcc :: run () , Command :: Fmt => fmt :: run () , Command :: Fuzz => fuzz :: run () , Command :: AbiTest => abi_test :: run () , } { eprintln ! ("Command failed to run: {e}") ; process :: exit (1) ; } }
};
}
