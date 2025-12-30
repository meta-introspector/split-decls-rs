// Generated macro for associated_type_shorthand_candidates (function)
macro_rules! Depcrate_lowerassociated_type_shorthand_candidates {
() => {
// Module: crate::lower
// Provides: {"associated_type_shorthand_candidates"}
// Dependencies: {}
pub fn associated_type_shorthand_candidates (db : & dyn HirDatabase , def : GenericDefId , res : TypeNs , mut cb : impl FnMut (& Name , TypeAliasId) -> bool ,) -> Option < TypeAliasId > { let interner = DbInterner :: new_with (db , None , None) ; named_associated_type_shorthand_candidates (interner , def , res , None , | name , _ , id | { cb (name , id) . then_some (id) }) }
};
}
