// Generated macro for format (function)
macro_rules! Depcrate_fmtformat {
() => {
// Module: crate::fmt
// Provides: {"format"}
// Dependencies: {}
# [doc = " Takes an [`Arguments`] struct and returns the resulting formatted string."] # [doc = ""] # [doc = " The [`Arguments`] instance can be created with the [`format_args!`] macro."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::fmt;"] # [doc = ""] # [doc = " let s = fmt::format(format_args!(\"Hello, {}!\", \"world\"));"] # [doc = " assert_eq!(s, \"Hello, world!\");"] # [doc = " ```"] # [doc = ""] # [doc = " Please note that using [`format!`] might be preferable."] # [doc = " Example:"] # [doc = ""] # [doc = " ```"] # [doc = " let s = format!(\"Hello, {}!\", \"world\");"] # [doc = " assert_eq!(s, \"Hello, world!\");"] # [doc = " ```"] # [doc = ""] # [doc = " [`format_args!`]: core::format_args"] # [doc = " [`format!`]: crate::format"] # [cfg (not (no_global_oom_handling))] # [must_use] # [stable (feature = "rust1" , since = "1.0.0")] # [inline] pub fn format (args : Arguments < '_ >) -> string :: String { fn format_inner (args : Arguments < '_ >) -> string :: String { let capacity = args . estimated_capacity () ; let mut output = string :: String :: with_capacity (capacity) ; output . write_fmt (args) . expect ("a formatting trait implementation returned an error when the underlying stream did not") ; output } args . as_str () . map_or_else (| | format_inner (args) , crate :: borrow :: ToOwned :: to_owned) }
};
}
