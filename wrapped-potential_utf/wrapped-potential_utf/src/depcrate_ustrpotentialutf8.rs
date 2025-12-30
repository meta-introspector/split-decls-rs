// Generated macro for PotentialUtf8 (struct)
macro_rules! Depcrate_ustrPotentialUtf8 {
() => {
// Module: crate::ustr
// Provides: {"PotentialUtf8"}
// Dependencies: {}
# [doc = " A byte slice that is expected to be a UTF-8 string but does not enforce that invariant."] # [doc = ""] # [doc = " Use this type instead of `str` if you don't need to enforce UTF-8 during deserialization. For"] # [doc = " example, strings that are keys of a map don't need to ever be reified as `str`s."] # [doc = ""] # [doc = " [`PotentialUtf8`] derefs to `[u8]`. To obtain a `str`, use [`Self::try_as_str()`]."] # [doc = ""] # [doc = " The main advantage of this type over `[u8]` is that it serializes as a string in"] # [doc = " human-readable formats like JSON."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Using an [`PotentialUtf8`] as the key of a [`ZeroMap`]:"] # [doc = ""] # [doc = " ```"] # [doc = " use potential_utf::PotentialUtf8;"] # [doc = " use zerovec::ZeroMap;"] # [doc = ""] # [doc = " // This map is cheap to deserialize, as we don't need to perform UTF-8 validation."] # [doc = " let map: ZeroMap<PotentialUtf8, u8> = ["] # [doc = "     (PotentialUtf8::from_bytes(b\"abc\"), 11),"] # [doc = "     (PotentialUtf8::from_bytes(b\"def\"), 22),"] # [doc = "     (PotentialUtf8::from_bytes(b\"ghi\"), 33),"] # [doc = " ]"] # [doc = " .into_iter()"] # [doc = " .collect();"] # [doc = ""] # [doc = " let key = \"abc\";"] # [doc = " let value = map.get_copied(PotentialUtf8::from_str(key));"] # [doc = " assert_eq!(Some(11), value);"] # [doc = " ```"] # [doc = ""] # [doc = " [`ZeroMap`]: zerovec::ZeroMap"] # [repr (transparent)] # [derive (PartialEq , Eq , PartialOrd , Ord)] # [allow (clippy :: exhaustive_structs)] pub struct PotentialUtf8 (pub [u8]) ;
};
}
