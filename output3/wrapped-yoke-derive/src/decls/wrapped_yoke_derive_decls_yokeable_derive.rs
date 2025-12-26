use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Custom derive for `yoke::Yokeable`,
///
/// If your struct contains `zerovec::ZeroMap`, then the compiler will not
/// be able to guarantee the lifetime covariance due to the generic types on
/// the `ZeroMap` itself. You must add the following attribute in order for
/// the custom derive to work with `ZeroMap`.
///
/// ```rust,ignore
/// #[derive(Yokeable)]
/// #[yoke(prove_covariance_manually)]
/// ```
///
/// Beyond this case, if the derive fails to compile due to lifetime issues, it
/// means that the lifetime is not covariant and `Yokeable` is not safe to implement.
#[proc_macro_derive(Yokeable, attributes(yoke))]
pub fn yokeable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(yokeable_derive_impl(&input))
}
