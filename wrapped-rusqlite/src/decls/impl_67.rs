macro_rules! deps {
    () => {
        Statement!();
        StatementCache!();
        CachedStatement!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl CachedStatement < '_ > { # [inline] fn new < 'conn > (stmt : Statement < 'conn > , cache : & 'conn StatementCache) -> CachedStatement < 'conn > { CachedStatement { stmt : Some (stmt) , cache , } } # [doc = " Discard the statement, preventing it from being returned to its"] # [doc = " [`Connection`]'s collection of cached statements."] # [inline] pub fn discard (mut self) { self . stmt = None ; } }
    };
}

impl_67!()