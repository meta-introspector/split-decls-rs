// Generated macro for ModKind (enum)
macro_rules! Depcrate_item_treeModKind {
() => {
// Module: crate::item_tree
// Provides: {"ModKind"}
// Dependencies: {}
# [derive (Debug , Clone , Eq , PartialEq)] pub (crate) enum ModKind { # [doc = " `mod m { ... }`"] Inline { items : Box < [ModItemId] > } , # [doc = " `mod m;`"] Outline , }
};
}
