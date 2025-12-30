// Generated macro for Module (struct)
macro_rules! Depcrate_vtabModule {
() => {
// Module: crate::vtab
// Provides: {"Module"}
// Dependencies: {}
# [doc = " Virtual table module"] # [doc = ""] # [doc = " (See [SQLite doc](https://sqlite.org/c3ref/module.html))"] # [repr (transparent)] pub struct Module < 'vtab , T : VTab < 'vtab > > { base : ffi :: sqlite3_module , phantom : PhantomData < & 'vtab T > , }
};
}
