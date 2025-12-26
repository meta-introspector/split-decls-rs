use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A type that can be used with turbofish syntax in [`Truncate::truncate`].
///
/// It is unlikely that you will want to use this trait directly. You are probably looking for the
/// [`Truncate`] trait.
pub trait TruncateTarget<T>: sealed::TruncateTargetSealed<T> {}
