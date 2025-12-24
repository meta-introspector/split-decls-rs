use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Provides the calling context to a closure called by `join_context`.
#[derive(Debug)]
pub struct FnContext {
    migrated: bool,
    /// disable `Send` and `Sync`, just for a little future-proofing.
    _marker: PhantomData<*mut ()>,
}
