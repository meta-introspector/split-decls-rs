// Generated macro for impl_31 (impl)
macro_rules! Depcrate_basicimpl_31 {
() => {
// Module: crate::basic
// Provides: {"impl_31"}
// Dependencies: {}
impl BasicClient { pub fn realm (& self) -> & str { & self . realm } # [doc = " Responds to the challenge with the supplied parameters."] # [doc = ""] # [doc = " This is functionally identical to [`encode_credentials`]; no parameters"] # [doc = " of the `BasicClient` are needed to produce the credentials."] # [inline] pub fn respond (& self , username : & str , password : & str) -> String { encode_credentials (username , password) } }
};
}
