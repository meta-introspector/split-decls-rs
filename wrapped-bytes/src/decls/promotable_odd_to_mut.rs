macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! promotable_odd_to_mut {
    () => {
        deps!();
        unsafe fn promotable_odd_to_mut (data : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> BytesMut { promotable_to_mut (data , ptr , len , | shared | shared . cast ()) }
    };
}

promotable_odd_to_mut!();