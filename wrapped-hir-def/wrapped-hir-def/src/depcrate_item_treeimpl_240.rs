// Generated macro for impl_240 (impl)
macro_rules! Depcrate_item_treeimpl_240 {
() => {
// Module: crate::item_tree
// Provides: {"impl_240"}
// Dependencies: {}
impl fmt :: Display for ImportAliasDisplay < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . value { ImportAlias :: Underscore => f . write_str ("_") , ImportAlias :: Alias (name) => fmt :: Display :: fmt (& name . display_no_db (self . edition) , f) , } } }
};
}
