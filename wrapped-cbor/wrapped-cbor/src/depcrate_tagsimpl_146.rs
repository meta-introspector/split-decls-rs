// Generated macro for impl_146 (impl)
macro_rules! Depcrate_tagsimpl_146 {
() => {
// Module: crate::tags
// Provides: {"impl_146"}
// Dependencies: {}
impl < T : Serialize > Serialize for Tagged < T > { fn serialize < S : Serializer > (& self , s : S) -> Result < S :: Ok , S :: Error > { set_tag (self . tag) ; let r = s . serialize_newtype_struct (CBOR_NEWTYPE_NAME , & self . value) ; set_tag (None) ; r } }
};
}
