// Generated macro for impl_79 (impl)
macro_rules! Depcrate_tokenizer_char_refimpl_79 {
() => {
// Module: crate::tokenizer::char_ref
// Provides: {"impl_79"}
// Dependencies: {}
impl CharRefTokenizer { pub fn new (addnl_allowed : Option < char >) -> CharRefTokenizer { CharRefTokenizer { state : Begin , addnl_allowed : addnl_allowed , result : None , num : 0 , num_too_big : false , seen_digit : false , hex_marker : None , name_buf_opt : None , name_match : None , name_len : 0 , } } pub fn get_result (self) -> CharRef { self . result . expect ("get_result called before done") } fn name_buf < 't > (& 't self) -> & 't StrTendril { self . name_buf_opt . as_ref () . expect ("name_buf missing in named character reference") } fn name_buf_mut < 't > (& 't mut self) -> & 't mut StrTendril { self . name_buf_opt . as_mut () . expect ("name_buf missing in named character reference") } fn finish_none (& mut self) -> Status { self . result = Some (CharRef { chars : ['\0' , '\0'] , num_chars : 0 , }) ; Done } fn finish_one (& mut self , c : char) -> Status { self . result = Some (CharRef { chars : [c , '\0'] , num_chars : 1 , }) ; Done } }
};
}
