use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The initializer for the [Shift_JIS](static.SHIFT_JIS.html) encoding.
///
/// For use only for taking the address of this form when
/// Rust prohibits the use of the non-`_INIT` form directly,
/// such as in initializers of other `static`s. If in doubt,
/// use the corresponding non-`_INIT` reference-typed `static`.
///
/// This part of the public API will go away if Rust changes
/// to make the referent of `pub const FOO: &'static Encoding`
/// unique cross-crate or if Rust starts allowing static arrays
/// to be initialized with `pub static FOO: &'static Encoding`
/// items.
pub static SHIFT_JIS_INIT: Encoding = Encoding {
    name: "Shift_JIS",
    variant: VariantEncoding::ShiftJis,
};
