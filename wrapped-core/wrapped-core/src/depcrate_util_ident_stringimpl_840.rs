// Generated macro for impl_840 (impl)
macro_rules! Depcrate_util_ident_stringimpl_840 {
() => {
// Module: crate::util::ident_string
// Provides: {"impl_840"}
// Dependencies: {}
impl IdentString { # [doc = " Create a new `IdentString`."] pub fn new (ident : Ident) -> Self { IdentString { string : ident . to_string () , ident , } } # [doc = " Get the ident as a `proc_macro2::Ident`."] pub fn as_ident (& self) -> & Ident { & self . ident } # [doc = " Get the ident as a string."] pub fn as_str (& self) -> & str { & self . string } # [doc = " Get the location of this `Ident` in source."] pub fn span (& self) -> Span { self . ident . span () } # [doc = " Apply some transform to the ident's string representation."] # [doc = ""] # [doc = " # Panics"] # [doc = " This will panic if the transform produces an invalid ident."] pub fn map < F , S > (self , map_fn : F) -> Self where F : FnOnce (String) -> S , S : AsRef < str > , { let span = self . span () ; let string = map_fn (self . string) ; Ident :: new (string . as_ref () , span) . into () } }
};
}
