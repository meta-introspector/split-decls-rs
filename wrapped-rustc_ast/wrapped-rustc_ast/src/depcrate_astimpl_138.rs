// Generated macro for impl_138 (impl)
macro_rules! Depcrate_astimpl_138 {
() => {
// Module: crate::ast
// Provides: {"impl_138"}
// Dependencies: {}
impl StrLit { pub fn as_token_lit (& self) -> token :: Lit { let token_kind = match self . style { StrStyle :: Cooked => token :: Str , StrStyle :: Raw (n) => token :: StrRaw (n) , } ; token :: Lit :: new (token_kind , self . symbol , self . suffix) } }
};
}
