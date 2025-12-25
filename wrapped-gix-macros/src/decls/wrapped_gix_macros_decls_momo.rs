use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// When applied to functions or methods, it will turn it into a wrapper that will immediately call
/// a de-monomorphized implementation (i.e. one that uses `&dyn Trait`).
///
/// That way, the landing-pads for convenience will be as small as possible which then delegate to a single
/// function or method for implementation.
///
/// The parameters using the following traits can be de-monomorphized:
///
/// * `Into`
/// * `AsRef`
/// * `AsMut`
#[proc_macro_attribute]
pub fn momo(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    momo::inner(input.into()).into()
}
