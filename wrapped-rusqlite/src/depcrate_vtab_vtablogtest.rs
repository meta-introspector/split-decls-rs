// Generated macro for test (module)
macro_rules! Depcrate_vtab_vtablogtest {
() => {
// Module: crate::vtab::vtablog
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: { Connection , Result } ; # [test] fn test_module () -> Result < () > { let db = Connection :: open_in_memory () ? ; super :: load_module (& db) ? ; db . execute_batch ("CREATE VIRTUAL TABLE temp.log USING vtablog(
                    schema='CREATE TABLE x(a,b,c)',
                    rows=3
                );" ,) ? ; let mut stmt = db . prepare ("SELECT * FROM log;") ? ; let mut rows = stmt . query ([]) ? ; while rows . next () ? . is_some () { } db . execute ("DELETE FROM log WHERE a = ?1" , ["a1"]) ? ; db . execute ("INSERT INTO log (a, b, c) VALUES (?1, ?2, ?3)" , ["a" , "b" , "c"] ,) ? ; db . execute ("UPDATE log SET b = ?1, c = ?2 WHERE a = ?3" , ["bn" , "cn" , "a1"] ,) ? ; db . query_one ("SELECT b, c FROM log WHERE a = 'a1'" , [] , | _ | Ok (0)) ? ; db . execute ("UPDATE log SET b = '' WHERE a IN (?1, ?2)" , ["a1" , "a2"]) ? ; Ok (()) } }
};
}
