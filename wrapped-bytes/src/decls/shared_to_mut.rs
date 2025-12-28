macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! shared_to_mut {
    () => {
        deps!();
        unsafe fn shared_to_mut (data : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> BytesMut { shared_to_mut_impl (data . load (Ordering :: Relaxed) . cast () , ptr , len) }
    };
}

shared_to_mut!();