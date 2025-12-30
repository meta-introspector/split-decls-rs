// Generated macro for impl_101 (impl)
macro_rules! Depcrate_typesimpl_101 {
() => {
// Module: crate::types
// Provides: {"impl_101"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for SourceItemOrderingModuleItemGroupings { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let groups = Vec :: < (String , Vec < SourceItemOrderingModuleItemKind >) > :: deserialize (deserializer) ? ; let items_total : usize = groups . iter () . map (| (_ , v) | v . len ()) . sum () ; let lut = Self :: build_lut (& groups) ; let back_lut = Self :: build_back_lut (& groups) ; let mut expected_items = SourceItemOrderingModuleItemKind :: all_variants () ; for item in lut . keys () { expected_items . retain (| i | i != item) ; } let all_items = SourceItemOrderingModuleItemKind :: all_variants () ; if expected_items . is_empty () && items_total == all_items . len () { let Some (use_group_index) = lut . get (& SourceItemOrderingModuleItemKind :: Use) else { return Err (de :: Error :: custom ("Error in internal LUT.")) ; } ; let Some ((_ , use_group_items)) = groups . get (* use_group_index) else { return Err (de :: Error :: custom ("Error in internal LUT.")) ; } ; if use_group_items . len () > 1 { return Err (de :: Error :: custom ("The group containing the \"use\" item kind may not contain any other item kinds. \
                    The \"use\" items will (generally) be sorted by rustfmt already. \
                    Therefore it makes no sense to implement linting rules that may conflict with rustfmt." ,)) ; } Ok (Self { groups , lut , back_lut }) } else if items_total != all_items . len () { Err (de :: Error :: custom (format ! ("Some module item kinds were configured more than once, or were missing, in the source ordering configuration. \
                The module item kinds are: {all_items:?}"))) } else { Err (de :: Error :: custom (format ! ("Not all module item kinds were part of the configured source ordering rule. \
                All item kinds must be provided in the config, otherwise the required source ordering would remain ambiguous. \
                The module item kinds are: {all_items:?}"))) } } }
};
}
