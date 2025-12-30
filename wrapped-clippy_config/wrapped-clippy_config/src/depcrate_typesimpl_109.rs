// Generated macro for impl_109 (impl)
macro_rules! Depcrate_typesimpl_109 {
() => {
// Module: crate::types
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for SourceItemOrderingTraitAssocItemKinds { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let items = Vec :: < SourceItemOrderingTraitAssocItemKind > :: deserialize (deserializer) ? ; let mut expected_items = SourceItemOrderingTraitAssocItemKind :: all_variants () ; for item in & items { expected_items . retain (| i | i != item) ; } let all_items = SourceItemOrderingTraitAssocItemKind :: all_variants () ; if expected_items . is_empty () && items . len () == all_items . len () { Ok (Self (items)) } else if items . len () != all_items . len () { Err (de :: Error :: custom (format ! ("Some trait associated item kinds were configured more than once, or were missing, in the source ordering configuration. \
                The trait associated item kinds are: {all_items:?}" ,))) } else { Err (de :: Error :: custom (format ! ("Not all trait associated item kinds were part of the configured source ordering rule. \
                All item kinds must be provided in the config, otherwise the required source ordering would remain ambiguous. \
                The trait associated item kinds are: {all_items:?}"))) } } }
};
}
