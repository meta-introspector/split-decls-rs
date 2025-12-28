macro_rules! SkipAnyWhileFolder {
    () => {
        struct SkipAnyWhileFolder < 'p , C , P > { base : C , predicate : & 'p P , skipping : & 'p AtomicBool , }
    };
}

SkipAnyWhileFolder!()