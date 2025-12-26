use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Singleton that all empty collections share.
/// Note: can't store non-zero ZSTs, we allocate in that case. We could
/// optimize everything to not do that (basically, make ptr == len and branch
/// on size == 0 in every method), but it's a bunch of work for something that
/// doesn't matter much.
#[cfg(any(not(feature = "gecko-ffi"), test, miri))]
static EMPTY_HEADER: Header = Header { _len: 0, _cap: 0 };
