// Generated macro for impl_53 (impl)
macro_rules! Depcrate_generatedimpl_53 {
() => {
// Module: crate::generated
// Provides: {"impl_53"}
// Dependencies: {}
impl LSMText { # [doc = " Adds an arbitrary binary token to the text. The order of tokens is"] # [doc = " significant if the map uses pairs or triplets, and the count of"] # [doc = " tokens is always significant."] # [doc (alias = "LSMTextAddToken")] # [inline] pub unsafe fn add_token (& self , token : & CFData) -> OSStatus { extern "C-unwind" { fn LSMTextAddToken (textref : & LSMText , token : & CFData) -> OSStatus ; } unsafe { LSMTextAddToken (self , token) } } }
};
}
