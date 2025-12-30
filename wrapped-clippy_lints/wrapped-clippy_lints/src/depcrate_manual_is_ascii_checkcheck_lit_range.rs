// Generated macro for check_lit_range (function)
macro_rules! Depcrate_manual_is_ascii_checkcheck_lit_range {
() => {
// Module: crate::manual_is_ascii_check
// Provides: {"check_lit_range"}
// Dependencies: {}
fn check_lit_range (start_lit : & Lit , end_lit : & Lit) -> CharRange { match (& start_lit . node , & end_lit . node) { (Char ('a') , Char ('z')) | (Byte (b'a') , Byte (b'z')) => CharRange :: LowerChar , (Char ('A') , Char ('Z')) | (Byte (b'A') , Byte (b'Z')) => CharRange :: UpperChar , (Char ('a') , Char ('f')) | (Byte (b'a') , Byte (b'f')) => CharRange :: LowerHexLetter , (Char ('A') , Char ('F')) | (Byte (b'A') , Byte (b'F')) => CharRange :: UpperHexLetter , (Char ('0') , Char ('9')) | (Byte (b'0') , Byte (b'9')) => CharRange :: Digit , _ => CharRange :: Otherwise , } }
};
}
