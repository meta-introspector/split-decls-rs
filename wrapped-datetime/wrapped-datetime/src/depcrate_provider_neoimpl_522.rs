// Generated macro for impl_522 (impl)
macro_rules! Depcrate_provider_neoimpl_522 {
() => {
// Module: crate::provider::neo
// Provides: {"impl_522"}
// Dependencies: {}
impl LinearNames < '_ > { # [doc = " Gets the 'am' name assuming this struct contains day period data."] pub (crate) fn am (& self) -> Option < & str > { self . names . get (0) } # [doc = " Gets the 'pm' name assuming this struct contains day period data."] pub (crate) fn pm (& self) -> Option < & str > { self . names . get (1) } # [doc = " Gets the 'noon' name assuming this struct contains day period data."] pub (crate) fn noon (& self) -> Option < & str > { self . names . get (2) . and_then (| s | if s . is_empty () { None } else { Some (s) }) } # [doc = " Gets the 'midnight' name assuming this struct contains day period data."] pub (crate) fn midnight (& self) -> Option < & str > { self . names . get (3) } }
};
}
