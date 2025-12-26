use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Extend<TokenStream> for TokenStream {
    fn extend<I: IntoIterator<Item = TokenStream>>(&mut self, streams: I) {
        self.inner
            .extend(streams.into_iter().map(|stream| stream.inner));
    }
}
