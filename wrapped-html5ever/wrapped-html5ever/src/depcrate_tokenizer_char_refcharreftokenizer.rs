// Generated macro for CharRefTokenizer (struct)
macro_rules! Depcrate_tokenizer_char_refCharRefTokenizer {
() => {
// Module: crate::tokenizer::char_ref
// Provides: {"CharRefTokenizer"}
// Dependencies: {}
pub struct CharRefTokenizer { state : State , addnl_allowed : Option < char > , result : Option < CharRef > , num : u32 , num_too_big : bool , seen_digit : bool , hex_marker : Option < char > , name_buf_opt : Option < StrTendril > , name_match : Option < (u32 , u32) > , name_len : usize , }
};
}
