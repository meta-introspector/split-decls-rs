// Generated macro for impl_96 (impl)
macro_rules! Depcrate_macro_optionsimpl_96 {
() => {
// Module: crate::macro_options
// Provides: {"impl_96"}
// Dependencies: {}
impl VisibilityAttr { pub fn to_explicit_visibility (& self) -> Option < Cow < '_ , syn :: Visibility > > { match self { Self :: Public (span) => Some (Cow :: Owned (syn :: Visibility :: Public (parse_quote_spanned ! (* span => pub) ,))) , Self :: Private => Some (Cow :: Owned (syn :: Visibility :: Inherited)) , Self :: Explicit (v) => Some (Cow :: Borrowed (v)) , Self :: None => None , } } }
};
}
