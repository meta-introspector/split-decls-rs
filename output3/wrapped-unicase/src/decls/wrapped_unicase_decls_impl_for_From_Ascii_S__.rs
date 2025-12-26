use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<S> From<Ascii<S>> for UniCase<S> {
    fn from(ascii: Ascii<S>) -> Self {
        UniCase(Encoding::Ascii(ascii))
    }
}
