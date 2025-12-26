use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An abstract stream of tokens, or more concretely a sequence of token trees.
///
/// This type provides interfaces for iterating over token trees and for
/// collecting token trees into one stream.
///
/// Token stream is both the input and output of `#[proc_macro]`,
/// `#[proc_macro_attribute]` and `#[proc_macro_derive]` definitions.
#[derive(Clone)]
pub struct TokenStream {
    inner: imp::TokenStream,
    _marker: ProcMacroAutoTraits,
}
