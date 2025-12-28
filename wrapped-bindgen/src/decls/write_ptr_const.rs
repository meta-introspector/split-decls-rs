macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! write_ptr_const {
    () => {
        deps!();
        fn write_ptr_const (pointers : usize) -> TokenStream { "*const " . repeat (pointers) . into () }
    };
}

write_ptr_const!();