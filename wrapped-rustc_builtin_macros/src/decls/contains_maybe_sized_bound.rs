macro_rules! contains_maybe_sized_bound {
    () => {
        fn contains_maybe_sized_bound (bounds : & [GenericBound]) -> bool { bounds . iter () . any (is_maybe_sized_bound) }
    };
}

contains_maybe_sized_bound!();