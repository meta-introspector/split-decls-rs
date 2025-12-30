// Generated macro for Tag (struct)
macro_rules! DepcrateTag {
() => {
// Module: crate
// Provides: {"Tag"}
// Dependencies: {}
# [doc = " A mutable git tag."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Tag { # [doc = " The hash this tag is pointing to."] pub target : gix_hash :: ObjectId , # [doc = " The kind of object this tag is pointing to."] pub target_kind : Kind , # [doc = " The name of the tag, e.g. \"v1.0\"."] pub name : BString , # [doc = " The tags author."] pub tagger : Option < gix_actor :: Signature > , # [doc = " The message describing the tag."] pub message : BString , # [doc = " A pgp signature over all bytes of the encoded tag, excluding the pgp signature itself."] pub pgp_signature : Option < BString > , }
};
}
