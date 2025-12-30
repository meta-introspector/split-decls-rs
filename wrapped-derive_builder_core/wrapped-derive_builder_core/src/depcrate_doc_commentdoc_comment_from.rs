// Generated macro for doc_comment_from (function)
macro_rules! Depcrate_doc_commentdoc_comment_from {
() => {
// Module: crate::doc_comment
// Provides: {"doc_comment_from"}
// Dependencies: {}
# [doc = " Doc-comment, implementing `quote::ToTokens`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Will expand to something like the following (depending on inner value):"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " # #[macro_use]"] # [doc = " # extern crate quote;"] # [doc = " # extern crate syn;"] # [doc = " # extern crate derive_builder_core;"] # [doc = " # use derive_builder_core::doc_comment_from;"] # [doc = " # fn main() {"] # [doc = " #    let doc_comment = doc_comment_from(\"foo\".to_string());"] # [doc = " #"] # [doc = " #    assert_eq!(quote!(#doc_comment).to_string(), quote!("] # [doc = " #[doc = \"foo\"]"] # [doc = " #    ).to_string());"] # [doc = " # }"] # [doc = " ```"] pub fn doc_comment_from (s : String) -> Attribute { parse_quote ! (# [doc =# s]) }
};
}
