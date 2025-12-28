macro_rules! static_is_unique {
    () => {
        fn static_is_unique (_ : & AtomicPtr < () >) -> bool { false }
    };
}

static_is_unique!()