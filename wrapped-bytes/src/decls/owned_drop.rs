macro_rules! owned_drop {
    () => {
        unsafe fn owned_drop < T > (data : & mut AtomicPtr < () > , _ptr : * const u8 , _len : usize) { let owned = data . load (Ordering :: Relaxed) ; owned_drop_impl :: < T > (owned) ; }
    };
}

owned_drop!();