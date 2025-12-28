macro_rules! deps {
    () => {
        ArrayTabCursor!();
    };
}

macro_rules! impl_616 {
    () => {
        deps!();
        impl ArrayTabCursor < '_ > { fn new < 'vtab > () -> ArrayTabCursor < 'vtab > { ArrayTabCursor { base : ffi :: sqlite3_vtab_cursor :: default () , row_id : 0 , ptr : None , phantom : PhantomData , } } fn len (& self) -> i64 { match self . ptr { Some (ref a) => a . len () as i64 , _ => 0 , } } }
    };
}

impl_616!()