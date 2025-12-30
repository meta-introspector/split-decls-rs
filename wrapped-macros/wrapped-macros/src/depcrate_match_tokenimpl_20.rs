// Generated macro for impl_20 (impl)
macro_rules! Depcrate_match_tokenimpl_20 {
() => {
// Module: crate::match_token
// Provides: {"impl_20"}
// Dependencies: {}
impl TagKind { # [doc = " Turn this `TagKind` into syntax for a literal `tokenizer::TagKind`."] fn lift (self , cx : & mut ExtCtxt) -> Tokens { match self { StartTag => quote_tokens ! (& mut * cx , :: tokenizer :: StartTag) , EndTag => quote_tokens ! (& mut * cx , :: tokenizer :: EndTag) , } } }
};
}
