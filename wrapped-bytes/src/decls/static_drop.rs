macro_rules! static_drop {
    () => {
        unsafe fn static_drop (_ : & mut AtomicPtr < () > , _ : * const u8 , _ : usize) { }
    };
}

static_drop!();