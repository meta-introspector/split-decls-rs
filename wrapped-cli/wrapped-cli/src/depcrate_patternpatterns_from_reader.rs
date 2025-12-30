// Generated macro for patterns_from_reader (function)
macro_rules! Depcrate_patternpatterns_from_reader {
() => {
// Module: crate::pattern
// Provides: {"patterns_from_reader"}
// Dependencies: {}
# [doc = " Read patterns from any reader, one per line."] # [doc = ""] # [doc = " If there was a problem reading or if any of the patterns contain invalid"] # [doc = " UTF-8, then an error is returned. If there was a problem with a specific"] # [doc = " pattern, then the error message will include the line number."] # [doc = ""] # [doc = " Note that this routine uses its own internal buffer, so the caller should"] # [doc = " not provide their own buffered reader if possible."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This shows how to parse patterns, one per line."] # [doc = ""] # [doc = " ```"] # [doc = " use grep_cli::patterns_from_reader;"] # [doc = ""] # [doc = " let patterns = \"\\"] # [doc = " foo"] # [doc = " bar\\\\s+foo"] # [doc = " [a-z]{3}"] # [doc = " \";"] # [doc = ""] # [doc = " assert_eq!(patterns_from_reader(patterns.as_bytes())?, vec!["] # [doc = "     r\"foo\","] # [doc = "     r\"bar\\s+foo\","] # [doc = "     r\"[a-z]{3}\","] # [doc = " ]);"] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub fn patterns_from_reader < R : io :: Read > (rdr : R) -> io :: Result < Vec < String > > { let mut patterns = vec ! [] ; let mut line_number = 0 ; io :: BufReader :: new (rdr) . for_byte_line (| line | { line_number += 1 ; match pattern_from_bytes (line) { Ok (pattern) => { patterns . push (pattern . to_string ()) ; Ok (true) } Err (err) => Err (io :: Error :: new (io :: ErrorKind :: Other , format ! ("{}: {}" , line_number , err) ,)) , } }) ? ; Ok (patterns) }
};
}
