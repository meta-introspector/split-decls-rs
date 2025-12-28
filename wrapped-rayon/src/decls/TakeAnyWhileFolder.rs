macro_rules! TakeAnyWhileFolder {
    () => {
        struct TakeAnyWhileFolder < 'p , C , P > { base : C , predicate : & 'p P , taking : & 'p AtomicBool , }
    };
}

TakeAnyWhileFolder!()