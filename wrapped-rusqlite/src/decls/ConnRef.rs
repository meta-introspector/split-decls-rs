macro_rules! deps {
    () => {
        Connection!();
    };
}

macro_rules! ConnRef {
    () => {
        deps!();
        # [doc = " Connection reference"] pub struct ConnRef < 's > { ptr : * mut ffi :: sqlite3 , phantom : PhantomData < & 's () > , }
    };
}

ConnRef!()