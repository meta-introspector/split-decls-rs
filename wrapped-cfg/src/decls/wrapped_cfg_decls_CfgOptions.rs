use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Configuration options used for conditional compilation on items with `cfg` attributes.
/// We have two kind of options in different namespaces: atomic options like `unix`, and
/// key-value options like `target_arch="x86"`.
///
/// Note that for key-value options, one key can have multiple values (but not none).
/// `feature` is an example. We have both `feature="foo"` and `feature="bar"` if features
/// `foo` and `bar` are both enabled. And here, we store key-value options as a set of tuple
/// of key and value in `key_values`.
///
/// See: <https://doc.rust-lang.org/reference/conditional-compilation.html#set-configuration-options>
#[derive(Clone, PartialEq, Eq)]
pub struct CfgOptions {
    enabled: FxHashSet<CfgAtom>,
}
