use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// True if `c` is valid as a first character of an identifier.
/// See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for
/// a formal definition of valid identifier name.
pub fn is_id_start(c: char) -> bool {
    c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
}
