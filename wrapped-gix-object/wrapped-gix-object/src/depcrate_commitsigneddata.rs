// Generated macro for SignedData (struct)
macro_rules! Depcrate_commitSignedData {
() => {
// Module: crate::commit
// Provides: {"SignedData"}
// Dependencies: {}
# [doc = " The raw commit data, parseable by [`CommitRef`] or [`Commit`], which was fed into a program to produce a signature."] # [doc = ""] # [doc = " See [`extract_signature()`](crate::CommitRefIter::signature()) for how to obtain it."] # [derive (PartialEq , Eq , Debug , Hash , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct SignedData < 'a > { # [doc = " The raw commit data that includes the signature."] data : & 'a [u8] , # [doc = " The byte range at which we find the signature. All but the signature is the data that was signed."] signature_range : Range < usize > , }
};
}
