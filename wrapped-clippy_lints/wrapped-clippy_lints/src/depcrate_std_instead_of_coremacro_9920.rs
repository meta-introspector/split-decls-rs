// Generated macro for macro_9920 (macro)
macro_rules! Depcrate_std_instead_of_coremacro_9920 {
() => {
// Module: crate::std_instead_of_core
// Provides: {"macro_9920"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Finds items imported through `std` when available through `alloc`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Crates which have `no_std` compatibility and require alloc may wish to ensure types are imported from"] # [doc = " alloc to ensure disabling `std` does not cause the crate to fail to compile. This lint is also useful"] # [doc = " for crates migrating to become `no_std` compatible."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::vec::Vec;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # extern crate alloc;"] # [doc = " use alloc::vec::Vec;"] # [doc = " ```"] # [clippy :: version = "1.64.0"] pub STD_INSTEAD_OF_ALLOC , restriction , "type is imported from std when available in alloc" }
};
}
