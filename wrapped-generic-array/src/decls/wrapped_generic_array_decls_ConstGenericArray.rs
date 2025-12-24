use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// [`GenericArray`] with a const-generic `usize` length, using the [`ConstArrayLength`] type alias for `N`.
///
/// To construct from a literal array, use [`from_array`](GenericArray::from_array).
///
/// Note that not all `N` values are valid due to limitations inherent to `typenum` and Rust. You
/// may need to combine [Const] with other typenum operations to get the desired length.
pub type ConstGenericArray<T, const N: usize> = GenericArray<T, ConstArrayLength<N>>;
