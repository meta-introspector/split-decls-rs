macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! static_to_mut {
    () => {
        deps!();
        unsafe fn static_to_mut (_ : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> BytesMut { let slice = slice :: from_raw_parts (ptr , len) ; BytesMut :: from (slice) }
    };
}

static_to_mut!();