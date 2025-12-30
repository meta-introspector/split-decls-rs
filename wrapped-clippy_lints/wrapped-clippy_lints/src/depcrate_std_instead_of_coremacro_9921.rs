// Generated macro for macro_9921 (macro)
macro_rules! Depcrate_std_instead_of_coremacro_9921 {
() => {
// Module: crate::std_instead_of_core
// Provides: {"macro_9921"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Finds items imported through `alloc` when available through `core`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Crates which have `no_std` compatibility and may optionally require alloc may wish to ensure types are"] # [doc = " imported from core to ensure disabling `alloc` does not cause the crate to fail to compile. This lint"] # [doc = " is also useful for crates migrating to become `no_std` compatible."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " The lint is only partially aware of the required MSRV for items that were originally in `std` but moved"] # [doc = " to `core`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # extern crate alloc;"] # [doc = " use alloc::slice::from_ref;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use core::slice::from_ref;"] # [doc = " ```"] # [clippy :: version = "1.64.0"] pub ALLOC_INSTEAD_OF_CORE , restriction , "type is imported from alloc when available in core" }
};
}
