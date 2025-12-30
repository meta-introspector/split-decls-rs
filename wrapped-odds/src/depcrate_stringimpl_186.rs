// Generated macro for impl_186 (impl)
macro_rules! Depcrate_stringimpl_186 {
() => {
// Module: crate::string
// Provides: {"impl_186"}
// Dependencies: {}
impl CharStr { # [doc = " Create a new string from `c`."] pub fn new (c : char) -> CharStr { let mut self_ = CharStr { buf : [0 ; 4] , len : c . len_utf8 () as u32 , } ; let _ = crate :: char :: encode_utf8 (c , & mut self_ . buf) ; self_ } }
};
}
