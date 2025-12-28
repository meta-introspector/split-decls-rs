macro_rules! Chars {
    () => {
        # [doc = " Parallel iterator over the characters of a string"] # [derive (Debug , Clone)] pub struct Chars < 'ch > { chars : & 'ch str , }
    };
}

Chars!()