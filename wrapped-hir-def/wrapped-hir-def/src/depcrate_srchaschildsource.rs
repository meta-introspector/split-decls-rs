// Generated macro for HasChildSource (trait)
macro_rules! Depcrate_srcHasChildSource {
() => {
// Module: crate::src
// Provides: {"HasChildSource"}
// Dependencies: {}
pub trait HasChildSource < ChildId > { type Value ; fn child_source (& self , db : & dyn DefDatabase) -> InFile < ArenaMap < ChildId , Self :: Value > > ; }
};
}
