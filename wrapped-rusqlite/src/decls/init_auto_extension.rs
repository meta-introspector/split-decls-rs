macro_rules! deps {
    () => {
        AutoExtension!();
        RawAutoExtension!();
        Connection!();
        Error!();
    };
}

macro_rules! init_auto_extension {
    () => {
        deps!();
        # [doc = " Bridge between `RawAutoExtension` and `AutoExtension`"] # [doc = ""] # [doc = " # Safety"] # [doc = " * Opening a database from an auto-extension handler will lead to"] # [doc = "   an endless recursion of the auto-handler triggering itself"] # [doc = "   indirectly for each newly-opened database."] # [doc = " * Results are undefined if the given db is closed by an auto-extension."] # [doc = " * The list of auto-extensions should not be manipulated from an auto-extension."] pub unsafe fn init_auto_extension (db : * mut ffi :: sqlite3 , pz_err_msg : * mut * mut c_char , ax : AutoExtension ,) -> c_int { let r = catch_unwind (| | { let c = Connection :: from_handle (db) ; c . and_then (ax) }) . unwrap_or_else (| _ | Err (Error :: UnwindingPanic)) ; match r { Err (e) => to_sqlite_error (& e , pz_err_msg) , _ => ffi :: SQLITE_OK , } }
    };
}

init_auto_extension!()