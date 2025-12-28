macro_rules! str_for_sqlite {
    () => {
        # [doc = " Returns `(string ptr, len as c_int, SQLITE_STATIC | SQLITE_TRANSIENT)`"] # [doc = " normally."] # [doc = " The `sqlite3_destructor_type` item is always `SQLITE_TRANSIENT` unless"] # [doc = " the string was empty (in which case it's `SQLITE_STATIC`, and the ptr is"] # [doc = " static)."] fn str_for_sqlite (s : & [u8] ,) -> (* const c_char , ffi :: sqlite3_uint64 , ffi :: sqlite3_destructor_type ,) { let len = s . len () ; let (ptr , dtor_info) = if len != 0 { (s . as_ptr () . cast :: < c_char > () , ffi :: SQLITE_TRANSIENT ()) } else { ("" . as_ptr () . cast :: < c_char > () , ffi :: SQLITE_STATIC ()) } ; (ptr , len as ffi :: sqlite3_uint64 , dtor_info) }
    };
}

str_for_sqlite!()