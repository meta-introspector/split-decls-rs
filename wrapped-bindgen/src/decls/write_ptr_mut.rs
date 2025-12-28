macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! write_ptr_mut {
    () => {
        deps!();
        fn write_ptr_mut (pointers : usize) -> TokenStream { "*mut " . repeat (pointers) . into () }
    };
}

write_ptr_mut!()