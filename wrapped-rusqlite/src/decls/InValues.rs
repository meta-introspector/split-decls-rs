macro_rules! deps {
    () => {
        Filters!();
    };
}

macro_rules! InValues {
    () => {
        deps!();
        # [doc = " IN values"] pub struct InValues < 'a > { list : * mut ffi :: sqlite3_value , phantom : PhantomData < Filters < 'a > > , first : bool , }
    };
}

InValues!()