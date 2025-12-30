// Generated macro for impl_94 (impl)
macro_rules! Depcrate_frontendimpl_94 {
() => {
// Module: crate::frontend
// Provides: {"impl_94"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , B > Pattern < B > where B : PatternBackend , B :: PlaceholderKeyCow < 'a > : FromStr , < B :: PlaceholderKeyCow < 'a > as FromStr > :: Err : fmt :: Debug , { # [doc = " Creates a pattern by parsing a syntax string."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu_pattern::Pattern;"] # [doc = " use icu_pattern::SinglePlaceholder;"] # [doc = ""] # [doc = " // Create a pattern from a valid string:"] # [doc = " Pattern::<SinglePlaceholder>::try_from_str(\"{0} days\", Default::default())"] # [doc = "     .expect(\"valid pattern\");"] # [doc = ""] # [doc = " // Error on an invalid pattern:"] # [doc = " Pattern::<SinglePlaceholder>::try_from_str(\"{0 days\", Default::default())"] # [doc = "     .expect_err(\"mismatched braces\");"] # [doc = " ```"] pub fn try_from_str (pattern : & str , options : ParserOptions) -> Result < Box < Self > , Error > { let parser = Parser :: new (pattern , options) ; let store = B :: try_from_items (parser) ? ; # [cfg (debug_assertions)] match B :: validate_store (core :: borrow :: Borrow :: borrow (& store)) { Ok (()) => () , Err (e) => { debug_assert ! (false , "{e:?} for pattern {pattern:?}") ; } } ; Ok (Self :: from_boxed_store_unchecked (store)) } }
};
}
