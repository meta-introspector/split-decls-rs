// Generated macro for run_rustfmt (function)
macro_rules! Depcraterun_rustfmt {
() => {
// Module: crate
// Provides: {"run_rustfmt"}
// Dependencies: {}
pub fn run_rustfmt (data : impl fmt :: Display) -> Vec < u8 > { use std :: io :: Write ; let mut child = Command :: new ("rustfmt") . stdin (Stdio :: piped ()) . stdout (Stdio :: piped ()) . spawn () . expect ("failed running rustfmt") ; let mut stdin = child . stdin . take () . expect ("failed to open stdin") ; write ! (stdin , "{data}") . expect ("failed writing") ; drop (stdin) ; let output = child . wait_with_output () . expect ("failed formatting") ; if ! output . status . success () { panic ! ("failed running rustfmt with exit code {}" , output . status) } output . stdout }
};
}
