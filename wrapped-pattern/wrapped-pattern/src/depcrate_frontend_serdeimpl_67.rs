// Generated macro for impl_67 (impl)
macro_rules! Depcrate_frontend_serdeimpl_67 {
() => {
// Module: crate::frontend::serde
// Provides: {"impl_67"}
// Dependencies: {}
impl < 'de , 'data , B : PatternBackend > Deserialize < 'de > for & 'data Pattern < B > where 'de : 'data , & 'data B :: Store : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { Err (< D :: Error as :: serde :: de :: Error > :: custom ("human readable format cannot be borrowed" ,)) } else { let store = < & B :: Store > :: deserialize (deserializer) ? ; B :: validate_store (store) . map_err (< D :: Error as :: serde :: de :: Error > :: custom) ? ; Ok (Pattern :: from_ref_store_unchecked (store)) } } }
};
}
