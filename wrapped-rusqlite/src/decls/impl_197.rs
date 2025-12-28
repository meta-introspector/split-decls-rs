macro_rules! deps {
    () => {
        ToSqlOutput!();
        Result!();
        Sql!();
        ValueRef!();
        ToSql!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl Sql { pub fn new () -> Self { Self { buf : String :: new () } } pub fn push_pragma (& mut self , schema_name : Option < & str > , pragma_name : & str) -> Result < () > { self . push_keyword ("PRAGMA") ? ; self . push_space () ; if let Some (schema_name) = schema_name { self . push_schema_name (schema_name) ; self . push_dot () ; } self . push_keyword (pragma_name) } pub fn push_keyword (& mut self , keyword : & str) -> Result < () > { if ! keyword . is_empty () && is_identifier (keyword) { self . buf . push_str (keyword) ; Ok (()) } else { Err (err ! (ffi :: SQLITE_MISUSE , "Invalid keyword \"{keyword}\"")) } } pub fn push_schema_name (& mut self , schema_name : & str) { self . push_identifier (schema_name) ; } pub fn push_identifier (& mut self , s : & str) { if is_identifier (s) { self . buf . push_str (s) ; } else { self . wrap_and_escape (s , '"') ; } } pub fn push_value (& mut self , value : & dyn ToSql) -> Result < () > { let value = value . to_sql () ? ; let value = match value { ToSqlOutput :: Borrowed (v) => v , ToSqlOutput :: Owned (ref v) => ValueRef :: from (v) , # [cfg (any (feature = "blob" , feature = "functions" , feature = "array"))] _ => { return Err (err ! (ffi :: SQLITE_MISUSE , "Unsupported value \"{value:?}\"")) ; } } ; match value { ValueRef :: Integer (i) => { self . push_int (i) ; } ValueRef :: Real (r) => { self . push_real (r) ; } ValueRef :: Text (s) => { let s = std :: str :: from_utf8 (s) ? ; self . push_string_literal (s) ; } _ => { return Err (err ! (ffi :: SQLITE_MISUSE , "Unsupported value \"{value:?}\"")) ; } } ; Ok (()) } pub fn push_string_literal (& mut self , s : & str) { self . wrap_and_escape (s , '\'') ; } pub fn push_int (& mut self , i : i64) { self . buf . push_str (& i . to_string ()) ; } pub fn push_real (& mut self , f : f64) { self . buf . push_str (& f . to_string ()) ; } pub fn push_space (& mut self) { self . buf . push (' ') ; } pub fn push_dot (& mut self) { self . buf . push ('.') ; } pub fn push_equal_sign (& mut self) { self . buf . push ('=') ; } pub fn open_brace (& mut self) { self . buf . push ('(') ; } pub fn close_brace (& mut self) { self . buf . push (')') ; } pub fn as_str (& self) -> & str { & self . buf } fn wrap_and_escape (& mut self , s : & str , quote : char) { self . buf . push (quote) ; let chars = s . chars () ; for ch in chars { if ch == quote { self . buf . push (ch) ; } self . buf . push (ch) ; } self . buf . push (quote) ; } }
    };
}

impl_197!()