// Generated macro for InvalidSetError (struct)
macro_rules! Depcrate_codepointinvlistInvalidSetError {
() => {
// Module: crate::codepointinvlist
// Provides: {"InvalidSetError"}
// Dependencies: {}
# [derive (Display , Debug)] # [doc = " A CodePointInversionList was constructed with an invalid inversion list"] # [cfg_attr (feature = "alloc" , displaydoc ("Invalid set: {0:?}"))] pub struct InvalidSetError (# [cfg (feature = "alloc")] pub alloc :: vec :: Vec < potential_utf :: PotentialCodePoint > ,) ;
};
}
