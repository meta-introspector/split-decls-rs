// Generated macro for impl_82 (impl)
macro_rules! Depcrate_features_rkyvimpl_82 {
() => {
// Module: crate::features::rkyv
// Provides: {"impl_82"}
// Dependencies: {}
impl < D : Fallible + ? Sized > Deserialize < CompactString , D > for ArchivedString where str : DeserializeUnsized < str , D > , { # [inline] fn deserialize (& self , _ : & mut D) -> Result < CompactString , D :: Error > { Ok (self . as_str () . into ()) } }
};
}
