macro_rules! next {
    () => {
        fn next (i : & BStr) -> (u8 , & BStr) { let b = i [0] ; (b , i [1 ..] . as_bstr ()) }
    };
}

next!();