// Generated macro for register_noargs (function)
macro_rules! Depcrate_sqlite_connection_functionsregister_noargs {
() => {
// Module: crate::sqlite::connection::functions
// Provides: {"register_noargs"}
// Dependencies: {}
pub (super) fn register_noargs < RetSqlType , Ret , F > (conn : & RawConnection , fn_name : & str , deterministic : bool , mut f : F ,) -> QueryResult < () > where F : FnMut () -> Ret + std :: panic :: UnwindSafe + Send + 'static , Ret : ToSql < RetSqlType , Sqlite > , Sqlite : HasSqlType < RetSqlType > , { conn . register_sql_function (fn_name , 0 , deterministic , move | _ , _ | Ok (f ())) ? ; Ok (()) }
};
}
