use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Associated [`ArrayLength`] for one [`Const<N>`]
///
/// See [`IntoArrayLength`] for more information.
///
/// Note that not all `N` values are valid due to limitations inherent to `typenum` and Rust. You
/// may need to combine [Const] with other typenum operations to get the desired length.
pub type ConstArrayLength<const N: usize> = <Const<N> as IntoArrayLength>::ArrayLength;
