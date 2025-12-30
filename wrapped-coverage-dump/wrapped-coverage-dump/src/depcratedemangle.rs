// Generated macro for demangle (function)
macro_rules! Depcratedemangle {
() => {
// Module: crate
// Provides: {"demangle"}
// Dependencies: {}
fn demangle () -> anyhow :: Result < () > { use std :: fmt :: Write as _ ; let stdin = std :: io :: read_to_string (std :: io :: stdin ()) ? ; let mut output = String :: with_capacity (stdin . len ()) ; for line in stdin . lines () { writeln ! (output , "{:#}" , rustc_demangle :: demangle (line)) ? ; } print ! ("{output}") ; Ok (()) }
};
}
