// Generated macro for as_trait_assoc_def (function)
macro_rules! Depcrate_traitsas_trait_assoc_def {
() => {
// Module: crate::traits
// Provides: {"as_trait_assoc_def"}
// Dependencies: {}
# [doc = " If this is an trait (impl) assoc item, returns the assoc item of the corresponding trait definition."] pub (crate) fn as_trait_assoc_def (db : & dyn HirDatabase , def : Definition) -> Option < Definition > { let assoc = def . as_assoc_item (db) ? ; let trait_ = match assoc . container (db) { hir :: AssocItemContainer :: Trait (_) => return Some (def) , hir :: AssocItemContainer :: Impl (i) => i . trait_ (db) , } ? ; assoc_item_of_trait (db , assoc , trait_) }
};
}
