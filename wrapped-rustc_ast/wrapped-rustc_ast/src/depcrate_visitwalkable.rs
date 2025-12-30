// Generated macro for Walkable (trait)
macro_rules! Depcrate_visitWalkable {
() => {
// Module: crate::visit
// Provides: {"Walkable"}
// Dependencies: {}
pub (crate) trait Walkable < 'a , V : Visitor < 'a > > { # [must_use] fn walk_ref (& 'a self , visitor : & mut V) -> V :: Result ; }
};
}
