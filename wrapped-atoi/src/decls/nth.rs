macro_rules! nth {
    () => {
        fn nth < I > (n : u8) -> I where I : Zero + One , { let mut i = I :: zero () ; for _ in 0 .. n { i = i + I :: one () ; } i }
    };
}

nth!()