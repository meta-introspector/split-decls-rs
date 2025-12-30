// Generated macro for superclasses_required_items (function)
macro_rules! Depcrate_stmtsuperclasses_required_items {
() => {
// Module: crate::stmt
// Provides: {"superclasses_required_items"}
// Dependencies: {}
pub (crate) fn superclasses_required_items < 'a , I > (superclasses : I ,) -> impl Iterator < Item = ItemTree > + 'a where I : IntoIterator < Item = ItemIdentifier > + 'a , < I as IntoIterator > :: IntoIter : Clone , { let iter = superclasses . into_iter () ; iter . clone () . enumerate () . map (move | (i , superclass) | { ItemTree :: new (superclass , superclasses_required_items (iter . clone () . skip (i + 1) . collect :: < Vec < _ > > ()) ,) }) . chain (iter :: once (ItemTree :: objc ("__macros__"))) }
};
}
