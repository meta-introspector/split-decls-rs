// Generated macro for impl_55 (impl)
macro_rules! Depcrate_styleimpl_55 {
() => {
// Module: crate::style
// Provides: {"impl_55"}
// Dependencies: {}
# [doc = " # Reflection"] impl Style { # [doc = " Get the foreground color"] # [inline] pub const fn get_fg_color (self) -> Option < crate :: Color > { self . fg } # [doc = " Get the background color"] # [inline] # [allow (missing_docs)] pub const fn get_bg_color (self) -> Option < crate :: Color > { self . bg } # [inline] # [allow (missing_docs)] pub const fn get_underline_color (self) -> Option < crate :: Color > { self . underline } # [inline] # [allow (missing_docs)] pub const fn get_effects (self) -> crate :: Effects { self . effects } # [doc = " Check if no styling is enabled"] # [inline] pub const fn is_plain (self) -> bool { self . fg . is_none () && self . bg . is_none () && self . underline . is_none () && self . effects . is_plain () } }
};
}
