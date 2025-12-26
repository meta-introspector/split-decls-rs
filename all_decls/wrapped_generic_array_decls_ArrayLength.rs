use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// `ArrayLength` is a type-level [`Unsigned`] integer used to
/// define the number of elements in a [`GenericArray`].
///
/// Consider `N: ArrayLength` to be equivalent to `const N: usize`
///
/// ```
/// # use generic_array::{GenericArray, ArrayLength};
/// fn foo<N: ArrayLength>(arr: GenericArray<i32, N>) -> i32 {
///     arr.iter().sum()
/// }
/// ```
/// is equivalent to:
/// ```
/// fn foo<const N: usize>(arr: [i32; N]) -> i32 {
///     arr.iter().sum()
/// }
/// ```
///
/// # Safety
///
/// This trait is effectively sealed due to only being allowed on [`Unsigned`] types,
/// and therefore cannot be implemented in user code.
///
/// Furthermore, this is limited to lengths less than or equal to `usize::MAX`.
/// ```compile_fail
/// # #![recursion_limit = "256"]
/// # use generic_array::{GenericArray, ArrayLength};
/// # use generic_array::typenum::{self, Unsigned};
/// type Empty = core::convert::Infallible; // Uninhabited ZST, size_of::<Empty>() == 0
///
/// // 2^64, greater than usize::MAX on 64-bit systems
/// type TooBig = typenum::operator_aliases::Shleft<typenum::U1, typenum::U64>;
///
/// // Compile Error due to ArrayLength not implemented for TooBig
/// let _ = GenericArray::<Empty, TooBig>::from_slice(&[]);
/// ```
pub unsafe trait ArrayLength: Unsigned + 'static {
    /// Associated type representing the underlying contiguous memory
    /// that constitutes an array with the given number of elements.
    ///
    /// This is an implementation detail, but is required to be public in cases where certain attributes
    /// of the inner type of [`GenericArray`] cannot be proven, such as [`Copy`] bounds.
    ///
    /// [`Copy`] example:
    /// ```
    /// # use generic_array::{GenericArray, ArrayLength};
    /// struct MyType<N: ArrayLength> {
    ///     data: GenericArray<f32, N>,
    /// }
    ///
    /// impl<N: ArrayLength> Clone for MyType<N> where N::ArrayType<f32>: Copy {
    ///     fn clone(&self) -> Self { MyType { ..*self } }
    /// }
    ///
    /// impl<N: ArrayLength> Copy for MyType<N> where N::ArrayType<f32>: Copy {}
    /// ```
    ///
    /// Alternatively, using the entire `GenericArray<f32, N>` type as the bounds works:
    /// ```ignore
    /// where GenericArray<f32, N>: Copy
    /// ```
    type ArrayType<T>: Sealed;
}
