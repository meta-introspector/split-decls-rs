// Generated macro for trait_item_visibility (function)
macro_rules! Depcrate_visibilitytrait_item_visibility {
() => {
// Module: crate::visibility
// Provides: {"trait_item_visibility"}
// Dependencies: {}
fn trait_item_visibility (db : & dyn DefDatabase , container : ItemContainerId) -> Option < Visibility > { match container { ItemContainerId :: TraitId (trait_) => Some (trait_visibility (db , trait_)) , _ => None , } }
};
}
