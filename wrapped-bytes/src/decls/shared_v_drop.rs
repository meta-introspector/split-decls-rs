macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! shared_v_drop {
    () => {
        deps!();
        unsafe fn shared_v_drop (data : & mut AtomicPtr < () > , _ptr : * const u8 , _len : usize) { data . with_mut (| shared | { release_shared (* shared as * mut Shared) ; }) ; }
    };
}

shared_v_drop!();