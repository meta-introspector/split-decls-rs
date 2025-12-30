// Generated macro for OwnerNode (enum)
macro_rules! Depcrate_hirOwnerNode {
() => {
// Module: crate::hir
// Provides: {"OwnerNode"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , HashStable_Generic)] pub enum OwnerNode < 'hir > { Item (& 'hir Item < 'hir >) , ForeignItem (& 'hir ForeignItem < 'hir >) , TraitItem (& 'hir TraitItem < 'hir >) , ImplItem (& 'hir ImplItem < 'hir >) , Crate (& 'hir Mod < 'hir >) , Synthetic , }
};
}
