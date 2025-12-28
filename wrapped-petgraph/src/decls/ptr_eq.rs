macro_rules! ptr_eq {
    () => {
        fn ptr_eq < T > (a : * const T , b : * const T) -> bool { a == b }
    };
}

ptr_eq!();