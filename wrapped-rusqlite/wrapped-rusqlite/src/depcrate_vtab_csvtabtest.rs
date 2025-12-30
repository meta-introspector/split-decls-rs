// Generated macro for test (module)
macro_rules! Depcrate_vtab_csvtabtest {
() => {
// Module: crate::vtab::csvtab
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: vtab :: csvtab ; use crate :: { Connection , Result } ; use fallible_iterator :: FallibleIterator ; # [test] fn test_csv_module () -> Result < () > { let db = Connection :: open_in_memory () ? ; csvtab :: load_module (& db) ? ; db . execute_batch ("CREATE VIRTUAL TABLE vtab USING csv(filename = 'test.csv', header = yes)" ,) ? ; { let mut s = db . prepare ("SELECT rowid, * FROM vtab") ? ; { let headers = s . column_names () ; assert_eq ! (vec ! ["rowid" , "colA" , "colB" , "colC"] , headers) ; } let ids : Result < Vec < i32 > > = s . query ([]) ? . map (| row | row . get :: < _ , i32 > (0)) . collect () ; let sum = ids ? . iter () . sum :: < i32 > () ; assert_eq ! (sum , 15) ; } db . execute_batch ("DROP TABLE vtab") } # [test] fn test_csv_cursor () -> Result < () > { let db = Connection :: open_in_memory () ? ; csvtab :: load_module (& db) ? ; db . execute_batch ("CREATE VIRTUAL TABLE vtab USING csv(filename='test.csv', header=yes)") ? ; { let mut s = db . prepare ("SELECT v1.rowid, v1.* FROM vtab v1 NATURAL JOIN vtab v2 WHERE \
                     v1.rowid < v2.rowid" ,) ? ; let mut rows = s . query ([]) ? ; let row = rows . next () ? . unwrap () ; assert_eq ! (row . get_unwrap ::< _ , i32 > (0) , 2) ; } db . execute_batch ("DROP TABLE vtab") } }
};
}
