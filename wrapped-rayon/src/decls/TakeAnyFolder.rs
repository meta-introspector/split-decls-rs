macro_rules! TakeAnyFolder {
    () => {
        struct TakeAnyFolder < 'f , C > { base : C , count : & 'f AtomicUsize , }
    };
}

TakeAnyFolder!();