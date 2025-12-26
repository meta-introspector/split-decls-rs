use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Encodes extra information we have about a pointer.
/// Note that this information is advisory only, and backends are free to ignore it:
/// if the information is wrong, that can cause UB, but if the information is absent,
/// that must always be okay.
#[derive(Copy, Clone, Debug)]
pub struct PointeeInfo {
    /// If this is `None`, then this is a raw pointer, so size and alignment are not guaranteed to
    /// be reliable.
    pub safe: Option<PointerKind>,
    /// If `safe` is `Some`, then the pointer is either null or dereferenceable for this many bytes.
    /// On a function argument, "dereferenceable" here means "dereferenceable for the entire duration
    /// of this function call", i.e. it is UB for the memory that this pointer points to be freed
    /// while this function is still running.
    /// The size can be zero if the pointer is not dereferenceable.
    pub size: Size,
    /// If `safe` is `Some`, then the pointer is aligned as indicated.
    pub align: Align,
}
