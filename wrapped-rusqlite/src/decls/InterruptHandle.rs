macro_rules! InterruptHandle {
    () => {
        # [doc = " Allows interrupting a long-running computation."] pub struct InterruptHandle { db_lock : Arc < Mutex < * mut ffi :: sqlite3 > > , }
    };
}

InterruptHandle!()