use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: MacroResult> MacroResult for Result<T> {
    fn into_result(self) -> Result<TokenStream> {
        match self {
            Ok(v) => v.into_result(),
            Err(err) => Err(err),
        }
    }
}
