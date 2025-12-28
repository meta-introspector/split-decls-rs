macro_rules! deps {
    () => {
        StringArray!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " A forward iterator over the strings of an array, casted to `&str`."] pub struct Iter < 'a > { range : Range < usize > , arr : & 'a StringArray , }
    };
}

Iter!();