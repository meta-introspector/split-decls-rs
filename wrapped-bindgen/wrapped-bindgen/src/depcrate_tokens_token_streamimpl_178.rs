// Generated macro for impl_178 (impl)
macro_rules! Depcrate_tokens_token_streamimpl_178 {
() => {
// Module: crate::tokens::token_stream
// Provides: {"impl_178"}
// Dependencies: {}
impl FromIterator < Self > for TokenStream { fn from_iter < I : IntoIterator < Item = Self > > (iter : I) -> Self { iter . into_iter () . fold (None , | accum : Option < Self > , n | { let mut ts = accum . unwrap_or_default () ; ts . combine (& n) ; Some (ts) }) . unwrap_or_else (Self :: new) } }
};
}
