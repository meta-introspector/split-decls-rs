macro_rules! is_weak_ref {
    () => {
        fn is_weak_ref (value : isize) -> bool { value < 0 }
    };
}

is_weak_ref!();