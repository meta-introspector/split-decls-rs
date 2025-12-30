// Generated macro for install_toolchain_or_exit (function)
macro_rules! Depcrateinstall_toolchain_or_exit {
() => {
// Module: crate
// Provides: {"install_toolchain_or_exit"}
// Dependencies: {}
fn install_toolchain_or_exit (versions : & Versions , name : & str) -> Result < () , Error > { eprintln ! ("[cargo-zerocopy] missing either toolchain '{name}' or component 'rust-src'") ; if env :: vars () . any (| v | v . 0 == "GITHUB_RUN_ID") { process :: exit (1) ; } loop { eprint ! ("[cargo-zerocopy] would you like to install toolchain '{name}' and component 'rust-src' via 'rustup' (y/n)? ") ; let mut input = [0] ; io :: stdin () . read_exact (& mut input) . unwrap () ; match input [0] as char { 'y' | 'Y' => break , 'n' | 'N' => process :: exit (1) , _ => () , } } let version = versions . get (name) ? ; rustup (["toolchain" , "install" , version , "-c" , "rust-src"] , None) . execute () ; Ok (()) }
};
}
