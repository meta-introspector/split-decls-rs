// Generated macro for test (module)
macro_rules! Depcrate_busytest {
() => {
// Module: crate::busy
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: { Connection , ErrorCode , Result , TransactionBehavior } ; use std :: sync :: atomic :: { AtomicBool , Ordering } ; # [test] fn test_default_busy () -> Result < () > { let temp_dir = tempfile :: tempdir () . unwrap () ; let path = temp_dir . path () . join ("test.db3") ; let mut db1 = Connection :: open (& path) ? ; let tx1 = db1 . transaction_with_behavior (TransactionBehavior :: Exclusive) ? ; let db2 = Connection :: open (& path) ? ; let r : Result < () > = db2 . query_row ("PRAGMA schema_version" , [] , | _ | unreachable ! ()) ; assert_eq ! (r . unwrap_err () . sqlite_error_code () , Some (ErrorCode :: DatabaseBusy)) ; tx1 . rollback () } # [test] fn test_busy_handler () -> Result < () > { static CALLED : AtomicBool = AtomicBool :: new (false) ; fn busy_handler (n : i32) -> bool { if n > 2 { false } else { CALLED . swap (true , Ordering :: Relaxed) } } let temp_dir = tempfile :: tempdir () . unwrap () ; let path = temp_dir . path () . join ("busy-handler.db3") ; let db1 = Connection :: open (& path) ? ; db1 . execute_batch ("CREATE TABLE IF NOT EXISTS t(a)") ? ; let db2 = Connection :: open (& path) ? ; db2 . busy_handler (Some (busy_handler)) ? ; db1 . execute_batch ("BEGIN EXCLUSIVE") ? ; let err = db2 . prepare ("SELECT * FROM t") . unwrap_err () ; assert_eq ! (err . sqlite_error_code () , Some (ErrorCode :: DatabaseBusy)) ; assert ! (CALLED . load (Ordering :: Relaxed)) ; db1 . busy_handler (None) ? ; Ok (()) } }
};
}
