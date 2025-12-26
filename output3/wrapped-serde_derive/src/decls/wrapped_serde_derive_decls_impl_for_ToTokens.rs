use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ToTokens for private {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(self.ident());
    }
}
