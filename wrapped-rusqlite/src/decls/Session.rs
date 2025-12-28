macro_rules! deps {
    () => {
        Filter!();
        Connection!();
    };
}

macro_rules! Session {
    () => {
        deps!();
        # [doc = " An instance of this object is a session that can be"] # [doc = " used to record changes to a database."] pub struct Session < 'conn > { phantom : PhantomData < & 'conn Connection > , s : * mut ffi :: sqlite3_session , filter : Filter , }
    };
}

Session!();