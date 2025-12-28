macro_rules! ptr_from_ref {
    () => {
        fn ptr_from_ref < T : ? Sized > (r : & T) -> * const T { r }
    };
}

ptr_from_ref!();