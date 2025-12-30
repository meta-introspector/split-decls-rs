// Generated macro for test (module)
macro_rules! Depcrate_serializetest {
() => {
// Module: crate::serialize
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: MAIN_DB ; # [test] fn serialize () -> Result < () > { let db = Connection :: open_in_memory () ? ; db . execute_batch ("CREATE TABLE x AS SELECT 'data'") ? ; let data = db . serialize (MAIN_DB) ? ; let Data :: Owned (data) = data else { panic ! ("expected OwnedData") } ; assert ! (data . sz > 0) ; Ok (()) } # [test] fn deserialize_read_exact () -> Result < () > { let db = Connection :: open_in_memory () ? ; db . execute_batch ("CREATE TABLE x AS SELECT 'data'") ? ; let data = db . serialize (MAIN_DB) ? ; let mut dst = Connection :: open_in_memory () ? ; let read = data . deref () ; dst . deserialize_read_exact (MAIN_DB , read , read . len () , false) ? ; dst . execute ("DELETE FROM x" , []) ? ; Ok (()) } # [test] fn deserialize_bytes () -> Result < () > { let data = b"" ; let mut dst = Connection :: open_in_memory () ? ; dst . deserialize_bytes (MAIN_DB , data) ? ; Ok (()) } # [test] fn deserialize () -> Result < () > { let src = Connection :: open_in_memory () ? ; src . execute_batch ("CREATE TABLE x AS SELECT 'data'") ? ; let data = src . serialize (MAIN_DB) ? ; let Data :: Owned (data) = data else { panic ! ("expected OwnedData") } ; let mut dst = Connection :: open_in_memory () ? ; dst . deserialize (MAIN_DB , data , false) ? ; dst . execute ("DELETE FROM x" , []) ? ; Ok (()) } }
};
}
