macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! ValueIter {
    () => {
        deps!();
        # [doc = " [`Values`] iterator."] pub struct ValueIter < 'a > { iter : slice :: Iter < 'a , * mut ffi :: sqlite3_value > , }
    };
}

ValueIter!();