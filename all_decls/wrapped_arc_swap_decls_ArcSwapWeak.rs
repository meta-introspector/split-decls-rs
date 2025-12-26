use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Arc swap for the [Weak] pointer.
///
/// This is similar to [ArcSwap], but it doesn't store [Arc], it stores [Weak]. It doesn't keep the
/// data alive when pointed to.
///
/// This is a type alias only. Most of the methods are described on the
/// [`ArcSwapAny`](struct.ArcSwapAny.html).
///
/// Needs the `weak` feature turned on.
///
/// [Weak]: std::sync::Weak
#[cfg(feature = "weak")]
pub type ArcSwapWeak<T> = ArcSwapAny<alloc::sync::Weak<T>>;
