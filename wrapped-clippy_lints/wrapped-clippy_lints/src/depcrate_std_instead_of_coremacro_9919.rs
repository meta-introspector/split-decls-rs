// Generated macro for macro_9919 (macro)
macro_rules! Depcrate_std_instead_of_coremacro_9919 {
() => {
// Module: crate::std_instead_of_core
// Provides: {"macro_9919"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Finds items imported through `std` when available through `core`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Crates which have `no_std` compatibility may wish to ensure types are imported from core to ensure"] # [doc = " disabling `std` does not cause the crate to fail to compile. This lint is also useful for crates"] # [doc = " migrating to become `no_std` compatible."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::hash::Hasher;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use core::hash::Hasher;"] # [doc = " ```"] # [clippy :: version = "1.64.0"] pub STD_INSTEAD_OF_CORE , restriction , "type is imported from std when available in core" }
};
}
