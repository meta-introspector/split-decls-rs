// Generated macro for U16UtfExt (trait)
macro_rules! Depcrate_traitsU16UtfExt {
() => {
// Module: crate::traits
// Provides: {"U16UtfExt"}
// Dependencies: {}
# [doc = " Methods for working with `u16`s as UTF-16 units."] pub trait U16UtfExt { # [doc = " Will you need an extra unit to complete this codepoint?"] # [doc = ""] # [doc = " Returns `Err` for trailing surrogates, `Ok(true)` for leading surrogates,"] # [doc = " and `Ok(false)` for others."] fn utf16_needs_extra_unit (self) -> Result < bool , Utf16FirstUnitError > ; # [doc = " Does this `u16` need another `u16` to complete a codepoint?"] # [doc = " Returns `(self & 0xfc00) == 0xd800`"] # [doc = ""] # [doc = " Is basically an unchecked variant of `utf16_needs_extra_unit()`."] fn is_utf16_leading_surrogate (self) -> bool ; }
};
}
