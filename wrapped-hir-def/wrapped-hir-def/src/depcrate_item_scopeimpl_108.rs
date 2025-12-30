// Generated macro for impl_108 (impl)
macro_rules! Depcrate_item_scopeimpl_108 {
() => {
// Module: crate::item_scope
// Provides: {"impl_108"}
// Dependencies: {}
impl ItemScope { pub (crate) fn update_visibility_types (& mut self , name : & Name , vis : Visibility) { let res = self . types . get_mut (name) . expect ("tried to update visibility of non-existent type") ; res . vis = vis ; } pub (crate) fn update_visibility_values (& mut self , name : & Name , vis : Visibility) { let res = self . values . get_mut (name) . expect ("tried to update visibility of non-existent value") ; res . vis = vis ; } pub (crate) fn update_visibility_macros (& mut self , name : & Name , vis : Visibility) { let res = self . macros . get_mut (name) . expect ("tried to update visibility of non-existent macro") ; res . vis = vis ; } }
};
}
