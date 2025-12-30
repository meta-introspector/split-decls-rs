// Generated macro for impl_53 (impl)
macro_rules! Depcrate_hyperlinkimpl_53 {
() => {
// Module: crate::hyperlink
// Provides: {"impl_53"}
// Dependencies: {}
impl HyperlinkAlias { # [doc = " Returns the name of the alias."] pub const fn name (& self) -> & str { self . name } # [doc = " Returns a very short description of this hyperlink alias."] pub const fn description (& self) -> & str { self . description } # [doc = " Returns the display priority of this alias."] # [doc = ""] # [doc = " If no priority is set, then `None` is returned."] # [doc = ""] # [doc = " The display priority is meant to reflect some special status associated"] # [doc = " with an alias. For example, the `default` and `none` aliases have a"] # [doc = " display priority. This is meant to encourage listing them first in"] # [doc = " documentation."] # [doc = ""] # [doc = " A lower display priority implies the alias should be shown before"] # [doc = " aliases with a higher (or absent) display priority."] # [doc = ""] # [doc = " Callers cannot rely on any specific display priority value to remain"] # [doc = " stable across semver compatible releases of this crate."] pub const fn display_priority (& self) -> Option < i16 > { self . display_priority } # [doc = " Returns the format string of the alias."] const fn format (& self) -> & 'static str { self . format } # [doc = " Looks for the hyperlink alias defined by the given name."] # [doc = ""] # [doc = " If one does not exist, `None` is returned."] fn find (name : & str) -> Option < & HyperlinkAlias > { HYPERLINK_PATTERN_ALIASES . binary_search_by_key (& name , | alias | alias . name ()) . map (| i | & HYPERLINK_PATTERN_ALIASES [i]) . ok () } }
};
}
