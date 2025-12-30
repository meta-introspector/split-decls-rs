// Generated macro for hash_derive_key_context (function)
macro_rules! Depcrate_hazmathash_derive_key_context {
() => {
// Module: crate::hazmat
// Provides: {"hash_derive_key_context"}
// Dependencies: {}
# [doc = " Hash a [`derive_key`](crate::derive_key) context string and return a [`ContextKey`]."] # [doc = ""] # [doc = " This has the same security requirement as [`derive_key`](crate::derive_key). **The context"] # [doc = " string should be hardcoded, globally unique, and application-specific.**"] # [doc = ""] # [doc = " The _only_ valid uses for the returned [`ContextKey`] are"] # [doc = " [`new_from_context_key`](HasherExt::new_from_context_key) and [`Mode::DeriveKeyMaterial`]"] # [doc = " (together with the merge subtree functions)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use blake3::Hasher;"] # [doc = " use blake3::hazmat::HasherExt;"] # [doc = ""] # [doc = " let context_key = blake3::hazmat::hash_derive_key_context(\"foo\");"] # [doc = " let mut hasher = Hasher::new_from_context_key(&context_key);"] # [doc = " hasher.update(b\"bar\");"] # [doc = " let derived_key = *hasher.finalize().as_bytes();"] # [doc = ""] # [doc = " assert_eq!(derived_key, blake3::derive_key(\"foo\", b\"bar\"));"] # [doc = " ```"] pub fn hash_derive_key_context (context : & str) -> ContextKey { crate :: hash_all_at_once :: < crate :: join :: SerialJoin > (context . as_bytes () , IV , crate :: DERIVE_KEY_CONTEXT ,) . root_hash () . 0 }
};
}
