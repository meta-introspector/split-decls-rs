macro_rules! deps {
    () => {
        Write!();
        CachedDate!();
        Result!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl fmt :: Write for CachedDate { fn write_str (& mut self , s : & str) -> fmt :: Result { let len = s . len () ; self . bytes [self . pos .. self . pos + len] . copy_from_slice (s . as_bytes ()) ; self . pos += len ; Ok (()) } }
    };
}

impl_54!()