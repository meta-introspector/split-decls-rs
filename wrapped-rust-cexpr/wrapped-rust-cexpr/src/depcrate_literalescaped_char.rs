// Generated macro for escaped_char (function)
macro_rules! Depcrate_literalescaped_char {
() => {
// Module: crate::literal
// Provides: {"escaped_char"}
// Dependencies: {}
fn escaped_char (i : & [u8]) -> nom :: IResult < & [u8] , CChar > { preceded (char ('\\') , alt ((map (one_of (r#"'"?\"#) , CChar :: Char) , map (one_of ("abfnrtv") , escape2char) , map_opt (many_m_n (1 , 3 , octal) , | v | c_raw_escape (v , 8)) , map_opt (preceded (char ('x') , many1 (hexadecimal)) , | v | { c_raw_escape (v , 16) }) , map_opt (preceded (char ('u') , many_m_n (4 , 4 , hexadecimal)) , c_unicode_escape ,) , map_opt (preceded (char ('U') , many_m_n (8 , 8 , hexadecimal)) , c_unicode_escape ,) ,)) ,) (i) }
};
}
