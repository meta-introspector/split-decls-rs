// Generated macro for test (module)
macro_rules! Depcrate_collationtest {
() => {
// Module: crate::collation
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: { Connection , Result } ; use fallible_streaming_iterator :: FallibleStreamingIterator ; use std :: cmp :: Ordering ; use unicase :: UniCase ; fn unicase_compare (s1 : & str , s2 : & str) -> Ordering { UniCase :: new (s1) . cmp (& UniCase :: new (s2)) } # [test] fn test_unicase () -> Result < () > { let db = Connection :: open_in_memory () ? ; db . create_collation (c"unicase" , unicase_compare) ? ; collate (db) } fn collate (db : Connection) -> Result < () > { db . execute_batch ("CREATE TABLE foo (bar);
             INSERT INTO foo (bar) VALUES ('Maße');
             INSERT INTO foo (bar) VALUES ('MASSE');" ,) ? ; let mut stmt = db . prepare ("SELECT DISTINCT bar COLLATE unicase FROM foo ORDER BY 1") ? ; let rows = stmt . query ([]) ? ; assert_eq ! (rows . count () ?, 1) ; Ok (()) } fn collation_needed (db : & Connection , collation_name : & str) -> Result < () > { if "unicase" == collation_name { db . create_collation (collation_name , unicase_compare) } else { Ok (()) } } # [test] fn test_collation_needed () -> Result < () > { let db = Connection :: open_in_memory () ? ; db . collation_needed (collation_needed) ? ; collate (db) } # [test] fn remove_collation () -> Result < () > { let db = Connection :: open_in_memory () ? ; db . create_collation (c"unicase" , unicase_compare) ? ; db . remove_collation (c"unicase") } }
};
}
