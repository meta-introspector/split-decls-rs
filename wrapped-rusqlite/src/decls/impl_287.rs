macro_rules! deps {
    () => {
        StmtRef!();
        StatementStatus!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl StmtRef < '_ > { fn new (ptr : * mut ffi :: sqlite3_stmt) -> Self { StmtRef { ptr , phantom : PhantomData , } } # [doc = " SQL text"] pub fn sql (& self) -> Cow < '_ , str > { unsafe { CStr :: from_ptr (ffi :: sqlite3_sql (self . ptr)) . to_string_lossy () } } # [doc = " Expanded SQL text"] pub fn expanded_sql (& self) -> Option < String > { unsafe { crate :: raw_statement :: expanded_sql (self . ptr) . map (| s | s . to_string_lossy () . to_string ()) } } # [doc = " Get the value for one of the status counters for this statement."] pub fn get_status (& self , status : StatementStatus) -> i32 { unsafe { crate :: raw_statement :: stmt_status (self . ptr , status , false) } } }
    };
}

impl_287!()