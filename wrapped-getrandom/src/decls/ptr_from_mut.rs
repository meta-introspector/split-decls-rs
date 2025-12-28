macro_rules! ptr_from_mut {
    () => {
        fn ptr_from_mut < T : ? Sized > (r : & mut T) -> * mut T { r }
    };
}

ptr_from_mut!()