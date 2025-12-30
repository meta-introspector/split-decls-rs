// Generated macro for rustfmt (function)
macro_rules! Depcrate_testsrustfmt {
() => {
// Module: crate::tests
// Provides: {"rustfmt"}
// Dependencies: {}
fn rustfmt (input : & str) -> String { let mut rustfmt = Command :: new ("rustfmt") ; rustfmt . stdin (Stdio :: piped ()) ; rustfmt . stdout (Stdio :: piped ()) ; rustfmt . stderr (Stdio :: inherit ()) ; let mut child = match rustfmt . spawn () { Ok (c) => c , Err (e) => { eprintln ! ("failed to spawn rustfmt: {e:?}") ; return input . to_string () ; } } ; let mut stdout = child . stdout . take () . unwrap () ; let stdout_thread = std :: thread :: spawn (move | | { let mut buf = String :: new () ; stdout . read_to_string (& mut buf) . unwrap () ; buf }) ; let mut stdin = child . stdin . take () . unwrap () ; stdin . write_all (input . as_bytes ()) . unwrap () ; drop (stdin) ; let stdout_string : String = stdout_thread . join () . unwrap () ; let exit = child . wait () . unwrap () ; if ! exit . success () { eprintln ! ("rustfmt terminated with failure status code") ; return input . to_string () ; } stdout_string }
};
}
