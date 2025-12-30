// Generated macro for impl_849 (impl)
macro_rules! Depcrate_vtabimpl_849 {
() => {
// Module: crate::vtab
// Provides: {"impl_849"}
// Dependencies: {}
impl Connection { # [doc = " Register a virtual table implementation."] # [doc = ""] # [doc = " Step 3 of [Creating New Virtual Table"] # [doc = " Implementations](https://sqlite.org/vtab.html#creating_new_virtual_table_implementations)."] # [inline] pub fn create_module < 'vtab , T : VTab < 'vtab > , M : Name > (& self , module_name : M , module : & 'static Module < 'vtab , T > , aux : Option < T :: Aux > ,) -> Result < () > { self . db . borrow_mut () . create_module (module_name , module , aux) } }
};
}
