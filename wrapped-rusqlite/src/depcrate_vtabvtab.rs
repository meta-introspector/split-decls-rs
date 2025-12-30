// Generated macro for VTab (trait)
macro_rules! Depcrate_vtabVTab {
() => {
// Module: crate::vtab
// Provides: {"VTab"}
// Dependencies: {}
# [doc = " Eponymous-only virtual table instance trait."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The first item in a struct implementing `VTab` must be"] # [doc = " `rusqlite::sqlite3_vtab`, and the struct must be `#[repr(C)]`."] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " #[repr(C)]"] # [doc = " struct MyTab {"] # [doc = "    /// Base class. Must be first"] # [doc = "    base: rusqlite::vtab::sqlite3_vtab,"] # [doc = "    /* Virtual table implementations will typically add additional fields */"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " (See [SQLite doc](https://sqlite.org/c3ref/vtab.html))"] pub unsafe trait VTab < 'vtab > : Sized { # [doc = " Client data passed to [`Connection::create_module`]."] type Aux ; # [doc = " Specific cursor implementation"] type Cursor : VTabCursor ; # [doc = " Establish a new connection to an existing virtual table."] # [doc = ""] # [doc = " (See [SQLite doc](https://sqlite.org/vtab.html#the_xconnect_method))"] fn connect (db : & mut VTabConnection , aux : Option < & Self :: Aux > , args : & [& [u8]] ,) -> Result < (String , Self) > ; # [doc = " Determine the best way to access the virtual table."] # [doc = " (See [SQLite doc](https://sqlite.org/vtab.html#the_xbestindex_method))"] fn best_index (& self , info : & mut IndexInfo) -> Result < () > ; # [doc = " Create a new cursor used for accessing a virtual table."] # [doc = " (See [SQLite doc](https://sqlite.org/vtab.html#the_xopen_method))"] fn open (& 'vtab mut self) -> Result < Self :: Cursor > ; }
};
}
