// Generated macro for parse_impl (function)
macro_rules! Depcrate_bytestrparse_impl {
() => {
// Module: crate::bytestr
// Provides: {"parse_impl"}
// Dependencies: {}
# [doc = " Precondition: input has to start with either `b\"` or `br`."] # [inline (never)] fn parse_impl (input : & str) -> Result < (Option < Vec < u8 > > , Option < u8 > , usize) , ParseError > { if input . starts_with ("br") { scan_raw_string (input , 2 , false , true) . map (| (num , start_suffix) | (None , Some (num) , start_suffix)) } else { unescape_string :: < Vec < u8 > > (input , 2 , false , true , true) . map (| (v , start_suffix) | (v , None , start_suffix)) } }
};
}
