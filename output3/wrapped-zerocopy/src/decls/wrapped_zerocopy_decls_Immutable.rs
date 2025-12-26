use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types which are free from interior mutability.
///
/// `T: Immutable` indicates that `T` does not permit interior mutation, except
/// by ownership or an exclusive (`&mut`) borrow.
///
/// # Implementation
///
/// **Do not implement this trait yourself!** Instead, use
/// [`#[derive(Immutable)]`][derive] (requires the `derive` Cargo feature);
/// e.g.:
///
/// ```
/// # use zerocopy_derive::Immutable;
/// #[derive(Immutable)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(Immutable)]
/// enum MyEnum {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(Immutable)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// This derive performs a sophisticated, compile-time safety analysis to
/// determine whether a type is `Immutable`.
///
/// # Safety
///
/// Unsafe code outside of this crate must not make any assumptions about `T`
/// based on `T: Immutable`. We reserve the right to relax the requirements for
/// `Immutable` in the future, and if unsafe code outside of this crate makes
/// assumptions based on `T: Immutable`, future relaxations may cause that code
/// to become unsound.
///
#[cfg_attr(
    feature = "derive",
    doc = "[derive]: zerocopy_derive::Immutable",
    doc = "[derive-analysis]: zerocopy_derive::Immutable#analysis"
)]
#[cfg_attr(
    not(feature = "derive"),
    doc = concat!(
        "[derive]: https://docs.rs/zerocopy/",
        env!("CARGO_PKG_VERSION"),
        "/zerocopy/derive.Immutable.html"
    ),
    doc = concat!(
        "[derive-analysis]: https://docs.rs/zerocopy/",
        env!("CARGO_PKG_VERSION"),
        "/zerocopy/derive.Immutable.html#analysis"
    ),
)]
#[cfg_attr(
    zerocopy_diagnostic_on_unimplemented_1_78_0,
    diagnostic::on_unimplemented(note = "Consider adding `#[derive(Immutable)]` to `{Self}`")
)]
pub unsafe trait Immutable {
    #[doc(hidden)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized;
}
