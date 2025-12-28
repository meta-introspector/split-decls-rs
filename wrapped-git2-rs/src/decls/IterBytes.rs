macro_rules! deps {
    () => {
        StringArray!();
    };
}

macro_rules! IterBytes {
    () => {
        deps!();
        # [doc = " A forward iterator over the strings of an array, casted to `&[u8]`."] pub struct IterBytes < 'a > { range : Range < usize > , arr : & 'a StringArray , }
    };
}

IterBytes!();