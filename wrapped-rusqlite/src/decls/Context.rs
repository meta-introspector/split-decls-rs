macro_rules! Context {
    () => {
        # [doc = " Context is used by [`VTabCursor::column`] to specify the"] # [doc = " cell value."] pub struct Context (* mut ffi :: sqlite3_context) ;
    };
}

Context!()