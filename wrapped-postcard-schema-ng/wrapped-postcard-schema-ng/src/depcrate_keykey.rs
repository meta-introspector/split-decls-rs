// Generated macro for Key (struct)
macro_rules! Depcrate_keyKey {
() => {
// Module: crate::key
// Provides: {"Key"}
// Dependencies: {}
# [doc = " The `Key` uniquely identifies what \"kind\" of message this is."] # [doc = ""] # [doc = " In order to generate it, `postcard-schema` takes two pieces of data:"] # [doc = ""] # [doc = " * a `&str` \"path\" URI, similar to how you would use URIs as part of an HTTP path"] # [doc = " * The schema of the message type itself, using the [`Schema`] trait"] # [doc = ""] # [doc = " [`Schema`]: crate::Schema"] # [doc = ""] # [doc = " Specifically, we use [`Fnv1a`](https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function),"] # [doc = " and produce a 64-bit digest, by first hashing the path, then hashing the"] # [doc = " schema. Fnv1a is a non-cryptographic hash function, designed to be reasonably"] # [doc = " efficient to compute even on small platforms like microcontrollers."] # [doc = ""] # [doc = " Changing **anything** about *either* of the path or the schema will produce"] # [doc = " a drastically different `Key` value."] # [cfg_attr (feature = "defmt-v0_3" , derive (defmt_v0_3 :: Format))] # [derive (PartialEq , Eq , PartialOrd , Ord , Clone , Copy , Serialize , Deserialize , Hash)] pub struct Key ([u8 ; 8]) ;
};
}
