macro_rules! deps {
    () => {
        ToSqlOutput!();
        Null!();
        Blob!();
        ValueRef!();
        ZeroBlob!();
        Array!();
    };
}

macro_rules! set_result {
    () => {
        deps!();
        # [inline] pub (super) unsafe fn set_result (ctx : * mut sqlite3_context , # [allow (unused_variables)] args : & [* mut sqlite3_value] , result : & ToSqlOutput < '_ > ,) { let value = match * result { ToSqlOutput :: Borrowed (v) => v , ToSqlOutput :: Owned (ref v) => ValueRef :: from (v) , # [cfg (feature = "blob")] ToSqlOutput :: ZeroBlob (len) => { return ffi :: sqlite3_result_zeroblob (ctx , len) ; } # [cfg (feature = "functions")] ToSqlOutput :: Arg (i) => { return ffi :: sqlite3_result_value (ctx , args [i]) ; } # [cfg (feature = "array")] ToSqlOutput :: Array (ref a) => { return ffi :: sqlite3_result_pointer (ctx , Rc :: into_raw (a . clone ()) as * mut c_void , ARRAY_TYPE , Some (free_array) ,) ; } } ; match value { ValueRef :: Null => ffi :: sqlite3_result_null (ctx) , ValueRef :: Integer (i) => ffi :: sqlite3_result_int64 (ctx , i) , ValueRef :: Real (r) => ffi :: sqlite3_result_double (ctx , r) , ValueRef :: Text (s) => { let (c_str , len , destructor) = str_for_sqlite (s) ; ffi :: sqlite3_result_text64 (ctx , c_str , len , destructor , ffi :: SQLITE_UTF8 as _) ; } ValueRef :: Blob (b) => { let length = b . len () ; if length == 0 { ffi :: sqlite3_result_zeroblob (ctx , 0) ; } else { ffi :: sqlite3_result_blob64 (ctx , b . as_ptr () . cast :: < c_void > () , length as ffi :: sqlite3_uint64 , ffi :: SQLITE_TRANSIENT () ,) ; } } } }
    };
}

set_result!()