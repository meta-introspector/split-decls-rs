// Generated macro for eponymous_only_module (function)
macro_rules! Depcrate_vtabeponymous_only_module {
() => {
// Module: crate::vtab
// Provides: {"eponymous_only_module"}
// Dependencies: {}
# [doc = " Create an eponymous only virtual table implementation."] # [doc = ""] # [doc = " Step 2 of [Creating New Virtual Table Implementations](https://sqlite.org/vtab.html#creating_new_virtual_table_implementations)."] # [must_use] pub fn eponymous_only_module < 'vtab , T : VTab < 'vtab > > () -> & 'static Module < 'vtab , T > { module ! ('vtab , T , T :: Cursor , None , None , None) }
};
}
