macro_rules! deps {
    () => {
        StatementCache!();
        Statement!();
    };
}

macro_rules! CachedStatement {
    () => {
        deps!();
        # [doc = " Cacheable statement."] # [doc = ""] # [doc = " Statement will return automatically to the cache by default."] # [doc = " If you want the statement to be discarded, call"] # [doc = " [`discard()`](CachedStatement::discard) on it."] pub struct CachedStatement < 'conn > { stmt : Option < Statement < 'conn > > , cache : & 'conn StatementCache , }
    };
}

CachedStatement!()