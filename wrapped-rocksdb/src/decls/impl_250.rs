macro_rules! deps {
    () => {
        DBPinnableSlice!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl Deref for DBPinnableSlice < '_ > { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { let mut val_len : size_t = 0 ; let val = ffi :: rocksdb_pinnableslice_value (self . ptr , & mut val_len) as * mut u8 ; slice :: from_raw_parts (val , val_len) } } }
    };
}

impl_250!()