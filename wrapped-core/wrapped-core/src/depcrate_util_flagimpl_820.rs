// Generated macro for impl_820 (impl)
macro_rules! Depcrate_util_flagimpl_820 {
() => {
// Module: crate::util::flag
// Provides: {"impl_820"}
// Dependencies: {}
impl Flag { # [doc = " Creates a new `Flag` which corresponds to the presence of a value."] pub fn present () -> Self { Flag (Some (Span :: call_site ())) } # [doc = " Check if the flag is present."] pub fn is_present (& self) -> bool { self . 0 . is_some () } # [deprecated (since = "0.14.0" , note = "Use Flag::is_present")] pub fn is_some (& self) -> bool { self . is_present () } # [doc = " Get the span of the flag, or [`Span::call_site`] if the flag was not present."] pub fn span (& self) -> Span { self . 0 . unwrap_or_else (Span :: call_site) } }
};
}
