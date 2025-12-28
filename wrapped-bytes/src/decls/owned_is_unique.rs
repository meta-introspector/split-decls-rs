macro_rules! owned_is_unique {
    () => {
        unsafe fn owned_is_unique (_data : & AtomicPtr < () >) -> bool { false }
    };
}

owned_is_unique!();