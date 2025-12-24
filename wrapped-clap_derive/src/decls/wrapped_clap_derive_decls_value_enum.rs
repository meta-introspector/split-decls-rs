use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Generates the `ValueEnum` impl.
#[proc_macro_derive(ValueEnum, attributes(clap, value))]
pub fn value_enum(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derives::derive_value_enum(&input)
        .unwrap_or_else(|err| {
            let dummy = dummies::value_enum(&input.ident);
            to_compile_error(err, dummy)
        })
        .into()
}
