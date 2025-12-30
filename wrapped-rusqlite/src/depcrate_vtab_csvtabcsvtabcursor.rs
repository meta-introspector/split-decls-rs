// Generated macro for CsvTabCursor (struct)
macro_rules! Depcrate_vtab_csvtabCsvTabCursor {
() => {
// Module: crate::vtab::csvtab
// Provides: {"CsvTabCursor"}
// Dependencies: {}
# [doc = " A cursor for the CSV virtual table"] # [repr (C)] struct CsvTabCursor < 'vtab > { # [doc = " Base class. Must be first"] base : ffi :: sqlite3_vtab_cursor , # [doc = " The CSV reader object"] reader : csv :: Reader < File > , # [doc = " Current cursor position used as rowid"] row_number : usize , # [doc = " Values of the current row"] cols : csv :: StringRecord , eof : bool , phantom : PhantomData < & 'vtab CsvTab > , }
};
}
