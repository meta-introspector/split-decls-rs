// Generated macro for TagRef (struct)
macro_rules! DepcrateTagRef {
() => {
// Module: crate
// Provides: {"TagRef"}
// Dependencies: {}
# [doc = " Represents a git tag, commonly indicating a software release."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct TagRef < 'a > { # [doc = " The hash in hexadecimal being the object this tag points to. Use [`target()`](TagRef::target()) to obtain a byte representation."] # [cfg_attr (feature = "serde" , serde (borrow))] pub target : & 'a BStr , # [doc = " The kind of object that `target` points to."] pub target_kind : Kind , # [doc = " The name of the tag, e.g. \"v1.0\"."] pub name : & 'a BStr , # [doc = " The raw tagger header value as encountered during parsing."] # [doc = ""] # [doc = " Use the [`tagger()`](TagRef::tagger()) method to obtain a parsed version of it."] # [cfg_attr (feature = "serde" , serde (borrow))] pub tagger : Option < & 'a BStr > , # [doc = " The message describing this release."] pub message : & 'a BStr , # [doc = " A cryptographic signature over the entire content of the serialized tag object thus far."] pub pgp_signature : Option < & 'a BStr > , }
};
}
