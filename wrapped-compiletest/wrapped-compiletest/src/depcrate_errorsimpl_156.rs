// Generated macro for impl_156 (impl)
macro_rules! Depcrate_errorsimpl_156 {
() => {
// Module: crate::errors
// Provides: {"impl_156"}
// Dependencies: {}
impl ErrorKind { pub fn from_compiler_str (s : & str) -> ErrorKind { match s { "help" => ErrorKind :: Help , "error" | "error: internal compiler error" => ErrorKind :: Error , "note" | "failure-note" => ErrorKind :: Note , "warning" => ErrorKind :: Warning , _ => panic ! ("unexpected compiler diagnostic kind `{s}`") , } } # [doc = " Either the canonical uppercase string, or some additional versions for compatibility."] # [doc = " FIXME: consider keeping only the canonical versions here."] fn from_user_str (s : & str) -> Option < ErrorKind > { Some (match s { "HELP" | "help" => ErrorKind :: Help , "ERROR" | "error" => ErrorKind :: Error , "NOTE" | "note" => ErrorKind :: Note , "SUGGESTION" => ErrorKind :: Suggestion , "WARN" | "WARNING" | "warn" | "warning" => ErrorKind :: Warning , "RAW" => ErrorKind :: Raw , _ => return None , }) } pub fn expect_from_user_str (s : & str) -> ErrorKind { ErrorKind :: from_user_str (s) . unwrap_or_else (| | { panic ! ("unexpected diagnostic kind `{s}`, expected \
                 `ERROR`, `WARN`, `NOTE`, `HELP`, `SUGGESTION` or `RAW`") }) } }
};
}
