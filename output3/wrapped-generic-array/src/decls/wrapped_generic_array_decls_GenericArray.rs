use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Struct representing a generic array - `GenericArray<T, N>` works like `[T; N]`
///
/// For how to implement [`Copy`] on structs using a generic-length `GenericArray` internally, see
/// the docs for [`ArrayLength::ArrayType`].
///
/// # Usage Notes
///
/// ### Initialization
///
/// Initialization of known-length `GenericArray`s can be done via the [`arr![]`](arr!) macro,
/// or [`from_array`](GenericArray::from_array)/[`from_slice`](GenericArray::from_slice).
///
/// For generic arrays of unknown/generic length, several safe methods are included to initialize
/// them, such as the [`GenericSequence::generate`] method:
///
/// ```rust
/// use generic_array::{GenericArray, sequence::GenericSequence, typenum, arr};
///
/// let evens: GenericArray<i32, typenum::U4> =
///            GenericArray::generate(|i: usize| i as i32 * 2);
///
/// assert_eq!(evens, arr![0, 2, 4, 6]);
/// ```
///
/// Furthermore, [`FromIterator`] and [`try_from_iter`](GenericArray::try_from_iter) exist to construct them
/// from iterators, but will panic/fail if not given exactly the correct number of elements.
///
/// ### Utilities
///
/// The [`GenericSequence`], [`FunctionalSequence`], [`Lengthen`], [`Shorten`], [`Split`], and [`Concat`] traits implement
/// some common operations on generic arrays.
///
/// ### Optimizations
///
/// Prefer to use the slice iterators like `.iter()`/`.iter_mut()` rather than by-value [`IntoIterator`]/[`GenericArrayIter`] if you can.
/// Slices optimize better. Using the [`FunctionalSequence`] methods also optimize well.
///
/// # How it works
///
/// The `typenum` crate uses Rust's type system to define binary integers as nested types,
/// and allows for operations which can be applied to those type-numbers, such as `Add`, `Sub`, etc.
///
/// e.g. `6` would be `UInt<UInt<UInt<UTerm, B1>, B1>, B0>`
///
/// `generic-array` uses this nested type to recursively allocate contiguous elements, statically.
/// The [`ArrayLength`] trait is implemented on `UInt<N, B0>`, `UInt<N, B1>` and `UTerm`,
/// which correspond to even, odd and zero numeric values, respectively.
/// Together, these three cover all cases of `Unsigned` integers from `typenum`.
/// For `UInt<N, B0>` and `UInt<N, B1>`, it peels away the highest binary digit and
/// builds up a recursive structure that looks almost like a binary tree.
/// Then, within `GenericArray`, the recursive structure is reinterpreted as a contiguous
/// chunk of memory and allowing access to it as a slice.
///
/// <details>
/// <summary><strong>Expand for internal structure demonstration</strong></summary>
///
/// For example, `GenericArray<T, U6>` more or less expands to (at compile time):
///
/// ```ignore
/// GenericArray {
///     // 6 = UInt<UInt<UInt<UTerm, B1>, B1>, B0>
///     data: EvenData {
///         // 3 = UInt<UInt<UTerm, B1>, B1>
///         left: OddData {
///             // 1 = UInt<UTerm, B1>
///             left: OddData {
///                 left: (),  // UTerm
///                 right: (), // UTerm
///                 data: T,   // Element 0
///             },
///             // 1 = UInt<UTerm, B1>
///             right: OddData {
///                 left: (),  // UTerm
///                 right: (), // UTerm
///                 data: T,   // Element 1
///             },
///             data: T        // Element 2
///         },
///         // 3 = UInt<UInt<UTerm, B1>, B1>
///         right: OddData {
///             // 1 = UInt<UTerm, B1>
///             left: OddData {
///                 left: (),  // UTerm
///                 right: (), // UTerm
///                 data: T,   // Element 3
///             },
///             // 1 = UInt<UTerm, B1>
///             right: OddData {
///                 left: (),  // UTerm
///                 right: (), // UTerm
///                 data: T,   // Element 4
///             },
///             data: T        // Element 5
///         }
///     }
/// }
/// ```
///
/// This has the added benefit of only being `log2(N)` deep, which is important for things like `Drop`
/// to avoid stack overflows, since we can't implement `Drop` manually.
///
/// Then, we take the contiguous block of data and cast it to `*const T` or `*mut T` and use it as a slice:
///
/// ```ignore
/// unsafe {
///     slice::from_raw_parts(
///         self as *const GenericArray<T, N> as *const T,
///         <N as Unsigned>::USIZE
///     )
/// }
/// ```
///
/// </details>
#[repr(transparent)]
pub struct GenericArray<T, N: ArrayLength> {
    #[allow(dead_code)]
    data: N::ArrayType<T>,
}
