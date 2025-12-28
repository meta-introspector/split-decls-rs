macro_rules! deps {
    () => {
        Result!();
        Connection!();
        RawStatement!();
        Null!();
        ValueRef!();
        Blob!();
        Statement!();
    };
}

macro_rules! impl_278 {
    () => {
        deps!();
        impl Statement < '_ > { # [inline] pub (super) fn new (conn : & Connection , stmt : RawStatement) -> Statement < '_ > { Statement { conn , stmt } } pub (super) fn value_ref (& self , col : usize) -> ValueRef < '_ > { let raw = unsafe { self . stmt . ptr () } ; match self . stmt . column_type (col) { ffi :: SQLITE_NULL => ValueRef :: Null , ffi :: SQLITE_INTEGER => { ValueRef :: Integer (unsafe { ffi :: sqlite3_column_int64 (raw , col as c_int) }) } ffi :: SQLITE_FLOAT => { ValueRef :: Real (unsafe { ffi :: sqlite3_column_double (raw , col as c_int) }) } ffi :: SQLITE_TEXT => { let s = unsafe { let text = ffi :: sqlite3_column_text (raw , col as c_int) ; let len = ffi :: sqlite3_column_bytes (raw , col as c_int) ; assert ! (! text . is_null () , "unexpected SQLITE_TEXT column type with NULL data") ; from_raw_parts (text . cast :: < u8 > () , len as usize) } ; ValueRef :: Text (s) } ffi :: SQLITE_BLOB => { let (blob , len) = unsafe { (ffi :: sqlite3_column_blob (raw , col as c_int) , ffi :: sqlite3_column_bytes (raw , col as c_int) ,) } ; assert ! (len >= 0 , "unexpected negative return from sqlite3_column_bytes") ; if len > 0 { assert ! (! blob . is_null () , "unexpected SQLITE_BLOB column type with NULL data") ; ValueRef :: Blob (unsafe { from_raw_parts (blob . cast :: < u8 > () , len as usize) }) } else { ValueRef :: Blob (& []) } } _ => unreachable ! ("sqlite3_column_type returned invalid value") , } } # [inline] pub (super) fn step (& self) -> Result < bool > { match self . stmt . step () { ffi :: SQLITE_ROW => Ok (true) , ffi :: SQLITE_DONE => Ok (false) , code => Err (self . conn . decode_result (code) . unwrap_err ()) , } } # [inline] pub (super) fn reset (& self) -> Result < () > { match self . stmt . reset () { ffi :: SQLITE_OK => Ok (()) , code => Err (self . conn . decode_result (code) . unwrap_err ()) , } } }
    };
}

impl_278!();