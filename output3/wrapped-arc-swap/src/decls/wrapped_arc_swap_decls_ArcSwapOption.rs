use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An atomic storage for `Option<Arc>`.
///
/// This is very similar to [`ArcSwap`](type.ArcSwap.html), but allows storing NULL values, which
/// is useful in some situations.
///
/// This is a type alias only. Most of the methods are described on
/// [`ArcSwapAny`](struct.ArcSwapAny.html). Even though the examples there often use `ArcSwap`,
/// they are applicable to `ArcSwapOption` with appropriate changes.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use arc_swap::ArcSwapOption;
///
/// let shared = ArcSwapOption::from(None);
/// assert!(shared.load_full().is_none());
/// assert!(shared.swap(Some(Arc::new(42))).is_none());
/// assert_eq!(42, **shared.load_full().as_ref().unwrap());
/// ```
pub type ArcSwapOption<T> = ArcSwapAny<Option<Arc<T>>>;
