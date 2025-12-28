macro_rules! deps {
    () => {
        Result!();
        Blob!();
        Connection!();
        Name!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl Connection { # [doc = " Open a handle to the BLOB located in `row_id`,"] # [doc = " `column`, `table` in database `db`."] # [doc = ""] # [doc = " # Failure"] # [doc = ""] # [doc = " Will return `Err` if `db`/`table`/`column` cannot be converted to a"] # [doc = " C-compatible string or if the underlying SQLite BLOB open call"] # [doc = " fails."] # [inline] pub fn blob_open < D : Name , N : Name > (& self , db : D , table : N , column : N , row_id : i64 , read_only : bool ,) -> Result < Blob < '_ > > { let c = self . db . borrow_mut () ; let mut blob = ptr :: null_mut () ; let db = db . as_cstr () ? ; let table = table . as_cstr () ? ; let column = column . as_cstr () ? ; let rc = unsafe { ffi :: sqlite3_blob_open (c . db () , db . as_ptr () , table . as_ptr () , column . as_ptr () , row_id , ! read_only as std :: ffi :: c_int , & mut blob ,) } ; c . decode_result (rc) . map (| _ | Blob { conn : self , blob , pos : 0 , }) } }
    };
}

impl_46!()