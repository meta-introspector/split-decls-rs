// Generated macro for associated_type_by_name_including_super_traits (function)
macro_rules! Depcrate_utilsassociated_type_by_name_including_super_traits {
() => {
// Module: crate::utils
// Provides: {"associated_type_by_name_including_super_traits"}
// Dependencies: {}
pub (super) fn associated_type_by_name_including_super_traits (db : & dyn HirDatabase , trait_ref : TraitRef , name : & Name ,) -> Option < (TraitRef , TypeAliasId) > { all_super_trait_refs (db , trait_ref , | t | { let assoc_type = t . hir_trait_id () . trait_items (db) . associated_type_by_name (name) ? ; Some ((t , assoc_type)) }) }
};
}
