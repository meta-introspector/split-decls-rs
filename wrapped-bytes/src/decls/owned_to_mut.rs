macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! owned_to_mut {
    () => {
        deps!();
        unsafe fn owned_to_mut < T > (data : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> BytesMut { BytesMut :: from_vec (owned_to_vec :: < T > (data , ptr , len)) }
    };
}

owned_to_mut!()