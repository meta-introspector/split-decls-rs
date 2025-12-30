// Generated macro for impl_93 (impl)
macro_rules! Depcrate_typesimpl_93 {
() => {
// Module: crate::types
// Provides: {"impl_93"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for SourceItemOrdering { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let items = Vec :: < SourceItemOrderingCategory > :: deserialize (deserializer) ? ; let mut items_set = std :: collections :: HashSet :: new () ; for item in & items { if items_set . contains (item) { return Err (de :: Error :: custom (format ! ("The category \"{item:?}\" was enabled more than once in the source ordering configuration."))) ; } items_set . insert (item) ; } Ok (Self (items)) } }
};
}
