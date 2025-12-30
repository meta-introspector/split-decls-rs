// Generated macro for impl_81 (impl)
macro_rules! Depcrate_features_rkyvimpl_81 {
() => {
// Module: crate::features::rkyv
// Provides: {"impl_81"}
// Dependencies: {}
impl < S : Fallible + ? Sized > Serialize < S > for CompactString where str : SerializeUnsized < S > , S :: Error : Source , { # [inline] fn serialize (& self , serializer : & mut S) -> Result < Self :: Resolver , S :: Error > { ArchivedString :: serialize_from_str (self . as_str () , serializer) } }
};
}
