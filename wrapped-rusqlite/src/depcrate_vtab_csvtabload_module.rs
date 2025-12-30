// Generated macro for load_module (function)
macro_rules! Depcrate_vtab_csvtabload_module {
() => {
// Module: crate::vtab::csvtab
// Provides: {"load_module"}
// Dependencies: {}
# [doc = " Register the \"csv\" module."] # [doc = " ```sql"] # [doc = " CREATE VIRTUAL TABLE vtab USING csv("] # [doc = "   filename=FILENAME -- Name of file containing CSV content"] # [doc = "   [, schema=SCHEMA] -- Alternative CSV schema. 'CREATE TABLE x(col1 TEXT NOT NULL, col2 INT, ...);'"] # [doc = "   [, header=YES|NO] -- First row of CSV defines the names of columns if \"yes\". Default \"no\"."] # [doc = "   [, columns=N] -- Assume the CSV file contains N columns."] # [doc = "   [, delimiter=C] -- CSV delimiter. Default ','."] # [doc = "   [, quote=C] -- CSV quote. Default '\"'. 0 means no quote."] # [doc = " );"] # [doc = " ```"] pub fn load_module (conn : & Connection) -> Result < () > { let aux : Option < () > = None ; conn . create_module (c"csv" , read_only_module :: < CsvTab > () , aux) }
};
}
