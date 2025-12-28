macro_rules! deps {
    () => {
        Connection!();
        CachedStatement!();
        Statement!();
        StatementCache!();
        RawStatement!();
        Result!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl StatementCache { # [doc = " Create a statement cache."] # [inline] pub fn with_capacity (capacity : usize) -> Self { Self (RefCell :: new (LruCache :: new (capacity))) } # [inline] fn set_capacity (& self , capacity : usize) { self . 0 . borrow_mut () . set_capacity (capacity) ; } fn get < 'conn > (& 'conn self , conn : & 'conn Connection , sql : & str ,) -> Result < CachedStatement < 'conn > > { let trimmed = sql . trim () ; let mut cache = self . 0 . borrow_mut () ; let stmt = match cache . remove (trimmed) { Some (raw_stmt) => Ok (Statement :: new (conn , raw_stmt)) , None => conn . prepare_with_flags (trimmed , PrepFlags :: SQLITE_PREPARE_PERSISTENT) , } ; stmt . map (| mut stmt | { stmt . stmt . set_statement_cache_key (trimmed) ; CachedStatement :: new (stmt , self) }) } fn cache_stmt (& self , mut stmt : RawStatement) { if stmt . is_null () { return ; } let mut cache = self . 0 . borrow_mut () ; stmt . clear_bindings () ; if let Some (sql) = stmt . statement_cache_key () { cache . insert (sql , stmt) ; } else { debug_assert ! (false , "bug in statement cache code, statement returned to cache that without key") ; } } # [inline] fn flush (& self) { let mut cache = self . 0 . borrow_mut () ; cache . clear () ; } }
    };
}

impl_68!()