use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Represents the input to the find_nix_rustc! macro.
/// Can be empty, or a single string literal for filtering.
mod macro_args {
    use syn::{
        parse::{Parse, ParseStream, Result as SynResult},
        LitStr,
    };
    pub struct FindRustcArgs {
        pub filter: Option<LitStr>,
    }
    impl Parse for FindRustcArgs {
        fn parse(input: ParseStream) -> SynResult<Self> {
            if input.is_empty() {
                Ok(FindRustcArgs { filter: None })
            } else {
                let filter_lit: LitStr = input.parse()?;
                Ok(FindRustcArgs {
                    filter: Some(filter_lit),
                })
            }
        }
    }
}
