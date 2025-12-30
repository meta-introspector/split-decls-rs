// Generated macro for impl_907 (impl)
macro_rules! Depcrate_vtab_csvtabimpl_907 {
() => {
// Module: crate::vtab::csvtab
// Provides: {"impl_907"}
// Dependencies: {}
impl CsvTabCursor < '_ > { fn new < 'vtab > (reader : csv :: Reader < File >) -> CsvTabCursor < 'vtab > { CsvTabCursor { base : ffi :: sqlite3_vtab_cursor :: default () , reader , row_number : 0 , cols : csv :: StringRecord :: new () , eof : false , phantom : PhantomData , } } # [doc = " Accessor to the associated virtual table."] fn vtab (& self) -> & CsvTab { unsafe { & * (self . base . pVtab as * const CsvTab) } } }
};
}
