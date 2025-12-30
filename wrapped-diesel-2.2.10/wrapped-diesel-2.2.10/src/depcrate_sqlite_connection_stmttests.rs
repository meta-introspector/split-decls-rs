// Generated macro for tests (module)
macro_rules! Depcrate_sqlite_connection_stmttests {
() => {
// Module: crate::sqlite::connection::stmt
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; use crate :: sql_types :: Text ; # [test] fn check_out_of_bounds_bind_does_not_panic_on_drop () { let mut conn = SqliteConnection :: establish (":memory:") . unwrap () ; let e = crate :: sql_query ("SELECT '?'") . bind :: < Text , _ > ("foo") . execute (& mut conn) ; assert ! (e . is_err ()) ; let e = e . unwrap_err () ; if let crate :: result :: Error :: DatabaseError (crate :: result :: DatabaseErrorKind :: Unknown , m) = e { assert_eq ! (m . message () , "column index out of range") ; } else { panic ! ("Wrong error returned") ; } } }
};
}
