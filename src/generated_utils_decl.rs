// Generated declarations for utils module
// Extracted from: ../rust/compiler/rustc_macros/src/diagnostics/utils.rs

impl<'ty> FieldInnerTy<'ty> {
    fn single_generic_type(ty: &Type) -> &Type { unimplemented!() }
}
impl<T> SetOnce<T> for SpannedOption<T> {
    fn set_once(&mut self, value: T, span: Span) { unimplemented!() }
    fn value(self) -> Option<T> { unimplemented!() }
    fn value_ref(&self) -> Option<&T> { unimplemented!() }
}
impl FromStr for Applicability {
    fn from_str(s: &str) -> Result<Self, Self::Err> { unimplemented!() }
}
impl quote::ToTokens for Applicability {
    fn to_tokens(&self, tokens: &mut TokenStream) { unimplemented!() }
}
impl FromStr for SuggestionKind {
    fn from_str(s: &str) -> Result<Self, Self::Err> { unimplemented!() }
}
impl fmt::Display for SuggestionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { unimplemented!() }
}
impl SuggestionKind {
    fn from_suffix(s: &str) -> Option<Self> { unimplemented!() }
}
impl SubdiagnosticVariant {
}
impl quote::IdentFragment for SubdiagnosticKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { unimplemented!() }
    fn span(&self) -> Option<proc_macro2::Span> { unimplemented!() }
}
}
