// Generated macro for test (module)
macro_rules! Depcrate_configtest {
() => {
// Module: crate::config
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: DbConfig ; use crate :: { Connection , Result } ; # [test] fn test_db_config () -> Result < () > { let db = Connection :: open_in_memory () ? ; let opposite = ! db . db_config (DbConfig :: SQLITE_DBCONFIG_ENABLE_FKEY) ? ; assert_eq ! (db . set_db_config (DbConfig :: SQLITE_DBCONFIG_ENABLE_FKEY , opposite) , Ok (opposite)) ; assert_eq ! (db . db_config (DbConfig :: SQLITE_DBCONFIG_ENABLE_FKEY) , Ok (opposite)) ; let opposite = ! db . db_config (DbConfig :: SQLITE_DBCONFIG_ENABLE_TRIGGER) ? ; assert_eq ! (db . set_db_config (DbConfig :: SQLITE_DBCONFIG_ENABLE_TRIGGER , opposite) , Ok (opposite)) ; assert_eq ! (db . db_config (DbConfig :: SQLITE_DBCONFIG_ENABLE_TRIGGER) , Ok (opposite)) ; Ok (()) } }
};
}
