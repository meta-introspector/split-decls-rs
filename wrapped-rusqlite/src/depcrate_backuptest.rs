// Generated macro for test (module)
macro_rules! Depcrate_backuptest {
() => {
// Module: crate::backup
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: { Backup , Progress } ; use crate :: { Connection , Result , MAIN_DB , TEMP_DB } ; use std :: time :: Duration ; # [test] fn backup_to_path () -> Result < () > { let src = Connection :: open_in_memory () ? ; src . execute_batch ("CREATE TABLE foo AS SELECT 42 AS x") ? ; let temp_dir = tempfile :: tempdir () . unwrap () ; let path = temp_dir . path () . join ("test.db3") ; fn progress (_ : Progress) { } src . backup (MAIN_DB , path . as_path () , Some (progress)) ? ; let mut dst = Connection :: open_in_memory () ? ; dst . restore (MAIN_DB , path , Some (progress)) ? ; Ok (()) } # [test] fn test_backup () -> Result < () > { let src = Connection :: open_in_memory () ? ; let sql = "BEGIN;
                   CREATE TABLE foo(x INTEGER);
                   INSERT INTO foo VALUES(42);
                   END;" ; src . execute_batch (sql) ? ; let mut dst = Connection :: open_in_memory () ? ; { let backup = Backup :: new (& src , & mut dst) ? ; backup . step (- 1) ? ; } assert_eq ! (42 , dst . one_column ::< i64 , _ > ("SELECT x FROM foo" , []) ?) ; src . execute_batch ("INSERT INTO foo VALUES(43)") ? ; { let backup = Backup :: new (& src , & mut dst) ? ; backup . run_to_completion (5 , Duration :: from_millis (250) , None) ? ; } let the_answer : i64 = dst . one_column ("SELECT SUM(x) FROM foo" , []) ? ; assert_eq ! (42 + 43 , the_answer) ; Ok (()) } # [test] fn test_backup_temp () -> Result < () > { let src = Connection :: open_in_memory () ? ; let sql = "BEGIN;
                   CREATE TEMPORARY TABLE foo(x INTEGER);
                   INSERT INTO foo VALUES(42);
                   END;" ; src . execute_batch (sql) ? ; let mut dst = Connection :: open_in_memory () ? ; { let backup = Backup :: new_with_names (& src , TEMP_DB , & mut dst , MAIN_DB) ? ; backup . step (- 1) ? ; } assert_eq ! (42 , dst . one_column ::< i64 , _ > ("SELECT x FROM foo" , []) ?) ; src . execute_batch ("INSERT INTO foo VALUES(43)") ? ; { let backup = Backup :: new_with_names (& src , TEMP_DB , & mut dst , MAIN_DB) ? ; backup . run_to_completion (5 , Duration :: from_millis (250) , None) ? ; } let the_answer : i64 = dst . one_column ("SELECT SUM(x) FROM foo" , []) ? ; assert_eq ! (42 + 43 , the_answer) ; Ok (()) } # [test] fn test_backup_attached () -> Result < () > { let src = Connection :: open_in_memory () ? ; let sql = "ATTACH DATABASE ':memory:' AS my_attached;
                   BEGIN;
                   CREATE TABLE my_attached.foo(x INTEGER);
                   INSERT INTO my_attached.foo VALUES(42);
                   END;" ; src . execute_batch (sql) ? ; let mut dst = Connection :: open_in_memory () ? ; { let backup = Backup :: new_with_names (& src , c"my_attached" , & mut dst , MAIN_DB) ? ; backup . step (- 1) ? ; } assert_eq ! (42 , dst . one_column ::< i64 , _ > ("SELECT x FROM foo" , []) ?) ; src . execute_batch ("INSERT INTO foo VALUES(43)") ? ; { let backup = Backup :: new_with_names (& src , c"my_attached" , & mut dst , MAIN_DB) ? ; backup . run_to_completion (5 , Duration :: from_millis (250) , None) ? ; } let the_answer : i64 = dst . one_column ("SELECT SUM(x) FROM foo" , []) ? ; assert_eq ! (42 + 43 , the_answer) ; Ok (()) } }
};
}
