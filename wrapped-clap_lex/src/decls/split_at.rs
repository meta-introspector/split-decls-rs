macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! split_at {
    () => {
        deps!();
        # [doc = " Split an `OsStr`"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `index` must be at a valid UTF-8 boundary"] pub (crate) unsafe fn split_at (os : & OsStr , index : usize) -> (& OsStr , & OsStr) { unsafe { let bytes = os . as_encoded_bytes () ; let (first , second) = bytes . split_at (index) ; (OsStr :: from_encoded_bytes_unchecked (first) , OsStr :: from_encoded_bytes_unchecked (second) ,) } }
    };
}

split_at!()