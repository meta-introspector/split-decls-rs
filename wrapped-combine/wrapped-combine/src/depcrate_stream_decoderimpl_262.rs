// Generated macro for impl_262 (impl)
macro_rules! Depcrate_stream_decoderimpl_262 {
() => {
// Module: crate::stream::decoder
// Provides: {"impl_262"}
// Dependencies: {}
impl < S , P , C > Decoder < S , P , C > { # [doc (hidden)] pub fn advance < R > (& mut self , read : & mut R , removed : usize) where C : CombineBuffer < R > , { self . buffer . advance (read , removed) } # [doc (hidden)] # [cfg (feature = "pin-project-lite")] pub fn advance_pin < R > (& mut self , read : Pin < & mut R > , removed : usize) where C : CombineBuffer < R > , { self . buffer . advance_pin (read , removed) ; } pub fn position (& self) -> & P { & self . position } # [doc (hidden)] pub fn __inner (& mut self) -> (& mut S , & mut P , & C , bool) { (& mut self . state , & mut self . position , & self . buffer , self . end_of_input ,) } }
};
}
