// SRC: ../rust/compiler/rustc_data_structures/src/aligned.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=10 */
use std::marker::PointeeSized;
use std::ptr::Alignment;

/// Returns the ABI-required minimum alignment of a type in bytes.
///
/// This is equivalent to [`align_of`], but also works for some unsized
/// types (e.g. slices or rustc's `List`s).
pub const fn align_of<T: ?Sized + Aligned>() -> Alignment {
    T::ALIGN
}
/* AST_META: AST_ID=2 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=14 */

/// A type with a statically known alignment.
///
/// # Safety
///
/// `Self::ALIGN` must be equal to the alignment of `Self`. For sized types it
/// is [`align_of::<Self>()`], for unsized types it depends on the type, for
/// example `[T]` has alignment of `T`.
///
/// [`align_of::<Self>()`]: align_of
pub unsafe trait Aligned: PointeeSized {
    /// Alignment of `Self`.
    const ALIGN: Alignment;
}
/* AST_META: AST_ID=3 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=4 */

unsafe impl<T> Aligned for T {
    const ALIGN: Alignment = Alignment::of::<Self>();
}
/* AST_META: AST_ID=4 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=4 */

unsafe impl<T> Aligned for [T] {
    const ALIGN: Alignment = Alignment::of::<T>();
}