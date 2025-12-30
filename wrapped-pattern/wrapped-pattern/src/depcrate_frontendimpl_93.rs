// Generated macro for impl_93 (impl)
macro_rules! Depcrate_frontendimpl_93 {
() => {
// Module: crate::frontend
// Provides: {"impl_93"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < B > Pattern < B > where B : PatternBackend , { # [doc = " Creates a pattern from an iterator of pattern items."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu_pattern::Pattern;"] # [doc = " use icu_pattern::PatternItemCow;"] # [doc = " use icu_pattern::SinglePlaceholder;"] # [doc = " use icu_pattern::SinglePlaceholderKey;"] # [doc = " use std::borrow::Cow;"] # [doc = ""] # [doc = " Pattern::<SinglePlaceholder>::try_from_items("] # [doc = "     ["] # [doc = "         PatternItemCow::Placeholder(SinglePlaceholderKey::Singleton),"] # [doc = "         PatternItemCow::Literal(Cow::Borrowed(\" days\")),"] # [doc = "     ]"] # [doc = "     .into_iter(),"] # [doc = " )"] # [doc = " .expect(\"valid pattern items\");"] # [doc = " ```"] pub fn try_from_items < 'a , I > (items : I) -> Result < Box < Self > , Error > where I : Iterator < Item = PatternItemCow < 'a , B :: PlaceholderKeyCow < 'a > > > , { let store = B :: try_from_items (items . map (Ok)) ? ; # [cfg (debug_assertions)] match B :: validate_store (core :: borrow :: Borrow :: borrow (& store)) { Ok (()) => () , Err (e) => { debug_assert ! (false , "{e:?}") ; } } ; Ok (Self :: from_boxed_store_unchecked (store)) } }
};
}
