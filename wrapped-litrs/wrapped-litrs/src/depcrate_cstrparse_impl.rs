// Generated macro for parse_impl (function)
macro_rules! Depcrate_cstrparse_impl {
() => {
// Module: crate::cstr
// Provides: {"parse_impl"}
// Dependencies: {}
# [doc = " Precondition: input has to start with either `b\"` or `br`."] # [inline (never)] fn parse_impl (input : & str) -> Result < (CString , Option < u8 > , usize) , ParseError > { let (vec , num_hashes , start_suffix) = if input . starts_with ("cr") { scan_raw_string (input , 2 , true , false) . map (| (num , start_suffix) | (None , Some (num) , start_suffix)) ? } else { unescape_string :: < Vec < u8 > > (input , 2 , true , true , false) . map (| (v , start_suffix) | (v , None , start_suffix)) ? } ; let inner_range = inner_range (num_hashes , start_suffix) ; let vec = vec . unwrap_or_else (| | input [inner_range] . as_bytes () . to_vec ()) ; let value = CString :: new (vec) . unwrap () ; Ok ((value , num_hashes , start_suffix)) }
};
}
