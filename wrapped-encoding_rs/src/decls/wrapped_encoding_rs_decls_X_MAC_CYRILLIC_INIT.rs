use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The initializer for the [x-mac-cyrillic](static.X_MAC_CYRILLIC.html) encoding.
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
pub static X_MAC_CYRILLIC_INIT: Encoding = Encoding {
    name: "x-mac-cyrillic",
    variant: VariantEncoding::SingleByte(
        &data::SINGLE_BYTE_DATA.x_mac_cyrillic,
        0x0430,
        96,
        31,
    ),
};
