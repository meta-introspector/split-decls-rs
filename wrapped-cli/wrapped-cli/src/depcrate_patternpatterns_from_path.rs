// Generated macro for patterns_from_path (function)
macro_rules! Depcrate_patternpatterns_from_path {
() => {
// Module: crate::pattern
// Provides: {"patterns_from_path"}
// Dependencies: {}
# [doc = " Read patterns from a file path, one per line."] # [doc = ""] # [doc = " If there was a problem reading or if any of the patterns contain invalid"] # [doc = " UTF-8, then an error is returned. If there was a problem with a specific"] # [doc = " pattern, then the error message will include the line number and the file"] # [doc = " path."] pub fn patterns_from_path < P : AsRef < Path > > (path : P) -> io :: Result < Vec < String > > { let path = path . as_ref () ; let file = std :: fs :: File :: open (path) . map_err (| err | { io :: Error :: new (io :: ErrorKind :: Other , format ! ("{}: {}" , path . display () , err) ,) }) ? ; patterns_from_reader (file) . map_err (| err | { io :: Error :: new (io :: ErrorKind :: Other , format ! ("{}:{}" , path . display () , err) ,) }) }
};
}
