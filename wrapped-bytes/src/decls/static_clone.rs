macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! static_clone {
    () => {
        deps!();
        unsafe fn static_clone (_ : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> Bytes { let slice = slice :: from_raw_parts (ptr , len) ; Bytes :: from_static (slice) }
    };
}

static_clone!();