// Generated macro for impl_76 (impl)
macro_rules! Depcrate_initializerimpl_76 {
() => {
// Module: crate::initializer
// Provides: {"impl_76"}
// Dependencies: {}
impl < 'a > Initializer < 'a > { # [doc = " To be used inside of `#struct_field: match self.#builder_field { ... }`"] fn match_some (& 'a self) -> MatchSome < 'a > { match self . builder_pattern { BuilderPattern :: Owned => MatchSome :: Move , BuilderPattern :: Mutable | BuilderPattern :: Immutable => MatchSome :: Clone { crate_root : self . crate_root , } , } } # [doc = " To be used inside of `#struct_field: match self.#builder_field { ... }`"] fn match_none (& 'a self) -> MatchNone < 'a > { match self . default_value { Some (expr) => MatchNone :: DefaultTo { expr , crate_root : self . crate_root , } , None => { if self . use_default_struct { MatchNone :: UseDefaultStructField (self . field_ident) } else { MatchNone :: ReturnError { crate_root : self . crate_root , field_name : self . field_ident . to_string () , span : self . custom_error_type_span , } } } } } fn default (& 'a self) -> TokenStream { let crate_root = self . crate_root ; match self . default_value { Some (expr) => expr . with_crate_root (crate_root) . into_token_stream () , None if self . use_default_struct => { let struct_ident = syn :: Ident :: new (DEFAULT_STRUCT_NAME , Span :: call_site ()) ; let field_ident = self . field_ident ; quote ! (# struct_ident .# field_ident) } None => { quote ! (# crate_root :: export :: core :: default :: Default :: default ()) } } } }
};
}
