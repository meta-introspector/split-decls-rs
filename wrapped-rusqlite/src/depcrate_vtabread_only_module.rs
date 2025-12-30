// Generated macro for read_only_module (function)
macro_rules! Depcrate_vtabread_only_module {
() => {
// Module: crate::vtab
// Provides: {"read_only_module"}
// Dependencies: {}
# [doc = " Create a read-only virtual table implementation."] # [doc = ""] # [doc = " Step 2 of [Creating New Virtual Table Implementations](https://sqlite.org/vtab.html#creating_new_virtual_table_implementations)."] # [must_use] pub fn read_only_module < 'vtab , T : CreateVTab < 'vtab > > () -> & 'static Module < 'vtab , T > { match T :: KIND { VTabKind :: EponymousOnly => eponymous_only_module () , VTabKind :: Eponymous => { module ! ('vtab , T , T :: Cursor , Some (rust_connect ::< T >) , Some (rust_disconnect ::< T >) , None) } _ => { module ! ('vtab , T , T :: Cursor , Some (rust_create ::< T >) , Some (rust_destroy ::< T >) , None) } } }
};
}
