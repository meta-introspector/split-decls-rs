// Generated macro for InteriorMut (struct)
macro_rules! Depcrate_tyInteriorMut {
() => {
// Module: crate::ty
// Provides: {"InteriorMut"}
// Dependencies: {}
# [doc = " Helper to check if given type has inner mutability such as [`std::cell::Cell`] or"] # [doc = " [`std::cell::RefCell`]."] # [derive (Default , Debug)] pub struct InteriorMut < 'tcx > { ignored_def_ids : FxHashSet < DefId > , ignore_pointers : bool , tys : FxHashMap < Ty < 'tcx > , Option < & 'tcx ty :: List < Ty < 'tcx > > > > , }
};
}
