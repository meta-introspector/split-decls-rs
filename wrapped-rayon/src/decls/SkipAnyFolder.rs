macro_rules! SkipAnyFolder {
    () => {
        struct SkipAnyFolder < 'f , C > { base : C , count : & 'f AtomicUsize , }
    };
}

SkipAnyFolder!()