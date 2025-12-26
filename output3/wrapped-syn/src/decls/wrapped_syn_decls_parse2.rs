use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Parse a proc-macro2 token stream into the chosen syntax tree node.
///
/// This function parses a `proc_macro2::TokenStream` which is commonly useful
/// when the input comes from a node of the Syn syntax tree, for example the
/// body tokens of a [`Macro`] node. When in a procedural macro parsing the
/// `proc_macro::TokenStream` provided by the compiler, use [`syn::parse`]
/// instead.
///
/// [`syn::parse`]: parse()
///
/// This function enforces that the input is fully parsed. If there are any
/// unparsed tokens at the end of the stream, an error is returned.
#[cfg(feature = "parsing")]
#[cfg_attr(docsrs, doc(cfg(feature = "parsing")))]
pub fn parse2<T: parse::Parse>(tokens: proc_macro2::TokenStream) -> Result<T> {
    parse::Parser::parse2(T::parse, tokens)
}
