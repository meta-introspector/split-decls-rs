// Generated macro for impl_66 (impl)
macro_rules! Depcrate_frontend_serdeimpl_66 {
() => {
// Module: crate::frontend::serde
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'de , 'data , B > Deserialize < 'de > for Box < Pattern < B > > where 'de : 'data , B : PatternBackend < Store = str > , B :: PlaceholderKeyCow < 'data > : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { Pattern :: < B > :: try_from_items (< HumanReadablePattern < B > > :: deserialize (deserializer) ? . into_iter () ,) } else { let store = Box :: < B :: Store > :: deserialize (deserializer) ? ; B :: validate_store (& store) . map (| () | Pattern :: < B > :: from_boxed_store_unchecked (store)) } . map_err (< D :: Error as :: serde :: de :: Error > :: custom) } }
};
}
