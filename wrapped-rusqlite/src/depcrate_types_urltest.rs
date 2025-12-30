// Generated macro for test (module)
macro_rules! Depcrate_types_urltest {
() => {
// Module: crate::types::url
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: { params , Connection , Error , Result } ; use url :: { ParseError , Url } ; fn checked_memory_handle () -> Result < Connection > { let db = Connection :: open_in_memory () ? ; db . execute_batch ("CREATE TABLE urls (i INTEGER, v TEXT)") ? ; Ok (db) } fn get_url (db : & Connection , id : i64) -> Result < Url > { db . one_column ("SELECT v FROM urls WHERE i = ?" , [id]) } # [test] fn test_sql_url () -> Result < () > { let db = & checked_memory_handle () ? ; let url0 = Url :: parse ("http://www.example1.com") . unwrap () ; let url1 = Url :: parse ("http://www.example1.com/👌") . unwrap () ; let url2 = "http://www.example2.com/👌" ; db . execute ("INSERT INTO urls (i, v) VALUES (0, ?1), (1, ?2), (2, ?3), (3, ?4)" , params ! [url0 , url1 , url2 , "illegal"] ,) ? ; assert_eq ! (get_url (db , 0) ?, url0) ; assert_eq ! (get_url (db , 1) ?, url1) ; let out_url2 : Url = get_url (db , 2) ? ; assert_eq ! (out_url2 , Url :: parse (url2) . unwrap ()) ; let err = get_url (db , 3) . unwrap_err () ; match err { Error :: FromSqlConversionFailure (_ , _ , e) => { assert_eq ! (* e . downcast ::< ParseError > () . unwrap () , ParseError :: RelativeUrlWithoutBase ,) ; } e => { panic ! ("Expected conversion failure, got {e}") ; } } Ok (()) } }
};
}
