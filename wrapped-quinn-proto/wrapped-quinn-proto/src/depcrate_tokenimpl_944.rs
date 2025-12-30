// Generated macro for impl_944 (impl)
macro_rules! Depcrate_tokenimpl_944 {
() => {
// Module: crate::token
// Provides: {"impl_944"}
// Dependencies: {}
impl TokenType { fn from_byte (n : u8) -> Option < Self > { use TokenType :: * ; [Retry , Validation] . into_iter () . find (| ty | * ty as u8 == n) } }
};
}
