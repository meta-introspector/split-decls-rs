// Generated macro for header_field_multi_line (function)
macro_rules! Depcrate_encodeheader_field_multi_line {
() => {
// Module: crate::encode
// Provides: {"header_field_multi_line"}
// Dependencies: {}
pub (crate) fn header_field_multi_line (name : & [u8] , value : & [u8] , out : & mut dyn io :: Write) -> io :: Result < () > { let mut lines = value . as_bstr () . lines_with_terminator () ; out . write_all (name) ? ; out . write_all (SPACE) ? ; if let Some (line) = lines . next () { out . write_all (line) ? ; } for line in lines { out . write_all (SPACE) ? ; out . write_all (line) ? ; } if ! value . ends_with_str (b"\n") { out . write_all (NL) ? ; } Ok (()) }
};
}
