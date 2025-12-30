// Generated macro for Replacer (trait)
macro_rules! Depcrate_re_unicodeReplacer {
() => {
// Module: crate::re_unicode
// Provides: {"Replacer"}
// Dependencies: {}
# [doc = " Replacer describes types that can be used to replace matches in a string."] pub trait Replacer { # [doc = " Returns a possibly owned string that is used to replace the match"] # [doc = " corresponding to the `caps` capture group."] # [doc = ""] # [doc = " The `'a` lifetime refers to the lifetime of a borrowed string when"] # [doc = " a new owned string isn't needed (e.g., for `NoExpand`)."] fn reg_replace (& mut self , caps : & Captures) -> Cow < str > ; # [doc = " Returns a possibly owned string that never needs expansion."] fn no_expand (& mut self) -> Option < Cow < str > > { None } }
};
}
