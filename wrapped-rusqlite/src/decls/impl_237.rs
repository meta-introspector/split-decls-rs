macro_rules! deps {
    () => {
        OwnedData!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl OwnedData { # [doc = " # Safety"] # [doc = ""] # [doc = " Caller must be certain that `ptr` is allocated by `sqlite3_malloc64`."] pub unsafe fn from_raw_nonnull (ptr : NonNull < u8 > , sz : usize) -> Self { Self { ptr , sz } } fn into_raw (self) -> (* mut u8 , usize) { let raw = (self . ptr . as_ptr () , self . sz) ; std :: mem :: forget (self) ; raw } }
    };
}

impl_237!()