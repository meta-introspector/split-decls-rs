use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// `#"abc"#`, `##"a"` (fewer closing), or even `#"a` (unterminated).
///
/// Can capture fewer closing hashes than starting hashes,
/// for more efficient lexing and better backwards diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GuardedStr {
    pub n_hashes: u32,
    pub terminated: bool,
    pub token_len: u32,
}
