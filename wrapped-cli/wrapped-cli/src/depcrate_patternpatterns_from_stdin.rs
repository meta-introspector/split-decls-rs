// Generated macro for patterns_from_stdin (function)
macro_rules! Depcrate_patternpatterns_from_stdin {
() => {
// Module: crate::pattern
// Provides: {"patterns_from_stdin"}
// Dependencies: {}
# [doc = " Read patterns from stdin, one per line."] # [doc = ""] # [doc = " If there was a problem reading or if any of the patterns contain invalid"] # [doc = " UTF-8, then an error is returned. If there was a problem with a specific"] # [doc = " pattern, then the error message will include the line number and the fact"] # [doc = " that it came from stdin."] pub fn patterns_from_stdin () -> io :: Result < Vec < String > > { let stdin = io :: stdin () ; let locked = stdin . lock () ; patterns_from_reader (locked) . map_err (| err | { io :: Error :: new (io :: ErrorKind :: Other , format ! ("<stdin>:{}" , err)) }) }
};
}
