// Generated macro for from_buf_inner (function)
macro_rules! Depcrate_pemfrom_buf_inner {
() => {
// Module: crate::pem
// Provides: {"from_buf_inner"}
// Dependencies: {}
# [cfg (feature = "std")] fn from_buf_inner (rd : & mut dyn io :: BufRead , line : & mut Vec < u8 > , b64buf : & mut Vec < u8 > ,) -> Result < Option < (SectionKind , Vec < u8 >) > , Error > { let mut section = None ; loop { line . clear () ; let len = read_until_newline (rd , line) . map_err (Error :: Io) ? ; let next_line = if len == 0 { None } else { Some (line . as_slice ()) } ; match read (next_line , & mut section , b64buf) { Ok (ControlFlow :: Break (opt)) => return Ok (opt) , Ok (ControlFlow :: Continue (())) => continue , Err (e) => return Err (e) , } } }
};
}
