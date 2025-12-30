// Generated macro for stdin_or_bail (function)
macro_rules! Depcrate_plumbing_mainstdin_or_bail {
() => {
// Module: crate::plumbing::main
// Provides: {"stdin_or_bail"}
// Dependencies: {}
fn stdin_or_bail () -> Result < std :: io :: BufReader < std :: io :: Stdin > > { use is_terminal :: IsTerminal ; if std :: io :: stdin () . is_terminal () { anyhow :: bail ! ("Refusing to read from standard input while a terminal is connected") } Ok (BufReader :: new (stdin ())) }
};
}
