// Generated macro for test (module)
macro_rules! Depcrate_unlock_notifytest {
() => {
// Module: crate::unlock_notify
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: { Connection , OpenFlags , Result , Transaction , TransactionBehavior } ; use std :: sync :: mpsc :: sync_channel ; use std :: thread ; use std :: time ; # [test] fn test_unlock_notify () -> Result < () > { let url = "file::memory:?cache=shared" ; let flags = OpenFlags :: SQLITE_OPEN_READ_WRITE | OpenFlags :: SQLITE_OPEN_URI ; let db1 = Connection :: open_with_flags (url , flags) ? ; db1 . execute_batch ("CREATE TABLE foo (x)") ? ; let (rx , tx) = sync_channel (0) ; let child = thread :: spawn (move | | { let mut db2 = Connection :: open_with_flags (url , flags) . unwrap () ; let tx2 = Transaction :: new (& mut db2 , TransactionBehavior :: Immediate) . unwrap () ; tx2 . execute_batch ("INSERT INTO foo VALUES (42)") . unwrap () ; rx . send (1) . unwrap () ; let ten_millis = time :: Duration :: from_millis (10) ; thread :: sleep (ten_millis) ; tx2 . commit () . unwrap () ; }) ; assert_eq ! (tx . recv () . unwrap () , 1) ; assert_eq ! (42 , db1 . one_column ::< i64 , _ > ("SELECT x FROM foo" , []) ?) ; child . join () . unwrap () ; Ok (()) } }
};
}
