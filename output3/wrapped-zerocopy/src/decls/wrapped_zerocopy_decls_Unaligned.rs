use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types with no alignment requirement.
///
/// If `T: Unaligned`, then `align_of::<T>() == 1`.
///
/// # Implementation
///
/// **Do not implement this trait yourself!** Instead, use
/// [`#[derive(Unaligned)]`][derive]; e.g.:
///
/// ```
/// # use zerocopy_derive::Unaligned;
/// #[derive(Unaligned)]
/// #[repr(C)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(Unaligned)]
/// #[repr(u8)]
/// enum MyEnum {
/// #   Variant0,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(Unaligned)]
/// #[repr(packed)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// This derive performs a sophisticated, compile-time safety analysis to
/// determine whether a type is `Unaligned`.
///
/// # Safety
///
/// *This section describes what is required in order for `T: Unaligned`, and
/// what unsafe code may assume of such types. If you don't plan on implementing
/// `Unaligned` manually, and you don't plan on writing unsafe code that
/// operates on `Unaligned` types, then you don't need to read this section.*
///
/// If `T: Unaligned`, then unsafe code may assume that it is sound to produce a
/// reference to `T` at any memory location regardless of alignment. If a type
/// is marked as `Unaligned` which violates this contract, it may cause
/// undefined behavior.
///
/// `#[derive(Unaligned)]` only permits [types which satisfy these
/// requirements][derive-analysis].
///
#[cfg_attr(
    feature = "derive",
    doc = "[derive]: zerocopy_derive::Unaligned",
    doc = "[derive-analysis]: zerocopy_derive::Unaligned#analysis"
)]
#[cfg_attr(
    not(feature = "derive"),
    doc = concat!(
        "[derive]: https://docs.rs/zerocopy/",
        env!("CARGO_PKG_VERSION"),
        "/zerocopy/derive.Unaligned.html"
    ),
    doc = concat!(
        "[derive-analysis]: https://docs.rs/zerocopy/",
        env!("CARGO_PKG_VERSION"),
        "/zerocopy/derive.Unaligned.html#analysis"
    ),
)]
#[cfg_attr(
    zerocopy_diagnostic_on_unimplemented_1_78_0,
    diagnostic::on_unimplemented(note = "Consider adding `#[derive(Unaligned)]` to `{Self}`")
)]
pub unsafe trait Unaligned {
    #[doc(hidden)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized;
}
