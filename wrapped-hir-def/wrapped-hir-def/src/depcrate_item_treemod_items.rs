// Generated macro for mod_items (macro)
macro_rules! Depcrate_item_treemod_items {
() => {
// Module: crate::item_tree
// Provides: {"mod_items"}
// Dependencies: {}
macro_rules ! mod_items { ($ mod_item : ident -> $ ($ typ : ident in $ fld : ident -> $ ast : ty) ,+ $ (,) ?) => { # [derive (Debug , Copy , Clone , Eq , PartialEq , Hash)] pub (crate) enum $ mod_item { $ ($ typ (FileAstId <$ ast >) ,) + } impl $ mod_item { pub (crate) fn ast_id (self) -> FileAstId < ast :: Item > { match self { $ ($ mod_item ::$ typ (it) => it . upcast ()) ,+ } } } $ (impl From < FileAstId <$ ast >> for $ mod_item { fn from (id : FileAstId <$ ast >) -> $ mod_item { ModItemId ::$ typ (id) } }) + $ (impl ItemTreeNode for $ typ { type Source = $ ast ; } impl Index < FileAstId <$ ast >> for ItemTree { type Output = $ typ ; # [allow (unused_imports)] fn index (& self , index : FileAstId <$ ast >) -> & Self :: Output { use BigModItem ::*; use SmallModItem ::*; match & self .$ fld [& index . upcast ()] { $ typ (item) => item , _ => panic ! ("expected item of type `{}` at index `{:?}`" , stringify ! ($ typ) , index) , } } }) + } ; }
};
}
