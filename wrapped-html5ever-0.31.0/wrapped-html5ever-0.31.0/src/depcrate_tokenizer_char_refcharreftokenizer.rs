// Generated macro for CharRefTokenizer (struct)
macro_rules! Depcrate_tokenizer_char_refCharRefTokenizer {
() => {
// Module: crate::tokenizer::char_ref
// Provides: {"CharRefTokenizer"}
// Dependencies: {}
pub (super) struct CharRefTokenizer { state : State , result : Option < CharRef > , is_consumed_in_attribute : bool , num : u32 , num_too_big : bool , seen_digit : bool , hex_marker : Option < char > , name_buf_opt : Option < StrTendril > , name_match : Option < (u32 , u32) > , name_len : usize , }
};
}
