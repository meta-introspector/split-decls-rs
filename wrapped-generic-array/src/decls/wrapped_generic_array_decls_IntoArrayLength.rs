use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Implemented for types which can have an associated [`ArrayLength`],
/// such as [`Const<N>`] for use with const-generics.
///
/// ```
/// use generic_array::{GenericArray, IntoArrayLength, ConstArrayLength, typenum::Const};
///
/// fn some_array_interopt<const N: usize>(value: [u32; N]) -> GenericArray<u32, ConstArrayLength<N>>
/// where
///     Const<N>: IntoArrayLength,
/// {
///     let ga = GenericArray::from(value);
///     // do stuff
///     ga
/// }
/// ```
///
/// This is mostly to simplify the `where` bounds, equivalent to:
///
/// ```
/// use generic_array::{GenericArray, ArrayLength, typenum::{Const, U, ToUInt}};
///
/// fn some_array_interopt<const N: usize>(value: [u32; N]) -> GenericArray<u32, U<N>>
/// where
///     Const<N>: ToUInt,
///     U<N>: ArrayLength,
/// {
///     let ga = GenericArray::from(value);
///     // do stuff
///     ga
/// }
/// ```
pub trait IntoArrayLength {
    /// The associated `ArrayLength`
    type ArrayLength: ArrayLength;
}
