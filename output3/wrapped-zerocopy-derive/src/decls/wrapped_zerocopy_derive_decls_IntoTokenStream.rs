use serde::{Deserialize, Serialize};
use std::collections::HashMap;
trait IntoTokenStream {
    fn into_ts(self) -> TokenStream;
}
