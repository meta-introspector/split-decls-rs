// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [tokio :: main] async fn main () -> anyhow :: Result < () > { let opts = Opts :: parse () ; if opts . version { return print_version () ; } let mut source = match opts . command . clone () { None | Some (Command :: Stdin) => Source :: stdin () , Some (Command :: Tcp { host , port , set_addr , }) => Source :: tcp (host , port , set_addr) . await ? , Some (Command :: Serial { path , baud , dtr }) => Source :: serial (path , baud , dtr) ? , } ; if opts . watch_elf { run_and_watch (opts , & mut source) . await } else { run (opts , & mut source) . await } }
};
}
