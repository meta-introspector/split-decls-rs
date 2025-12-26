use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Extend<TokenTree> for TokenStream {
    fn extend<I: IntoIterator<Item = TokenTree>>(&mut self, tokens: I) {
        self.inner.extend(tokens);
    }
}
