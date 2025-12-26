use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Derive an extension trait for a given impl block. The trait name
/// goes into the parenthesized args of the macro, for greppability.
/// For example:
/// ```
/// use rustc_macros::extension;
/// #[extension(pub trait Foo)]
/// impl i32 { fn hello() {} }
/// ```
///
/// expands to:
/// ```
/// pub trait Foo { fn hello(); }
/// impl Foo for i32 { fn hello() {} }
/// ```
#[proc_macro_attribute]
pub fn extension(attr: TokenStream, input: TokenStream) -> TokenStream {
    extension::extension(attr, input)
}
