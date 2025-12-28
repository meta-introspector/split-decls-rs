macro_rules! deps {
    () => {
        Statement!();
    };
}

macro_rules! StmtRef {
    () => {
        deps!();
        # [doc = " Statement reference"] pub struct StmtRef < 's > { ptr : * mut ffi :: sqlite3_stmt , phantom : PhantomData < & 's () > , }
    };
}

StmtRef!()