// Generated macro for CsvTab (struct)
macro_rules! Depcrate_vtab_csvtabCsvTab {
() => {
// Module: crate::vtab::csvtab
// Provides: {"CsvTab"}
// Dependencies: {}
# [doc = " An instance of the CSV virtual table"] # [repr (C)] struct CsvTab { # [doc = " Base class. Must be first"] base : ffi :: sqlite3_vtab , # [doc = " Name of the CSV file"] filename : String , has_headers : bool , delimiter : u8 , quote : u8 , # [doc = " Offset to start of data"] offset_first_row : csv :: Position , }
};
}
