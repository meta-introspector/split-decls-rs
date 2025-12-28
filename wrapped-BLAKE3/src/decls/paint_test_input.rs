macro_rules! paint_test_input {
    () => {
        pub fn paint_test_input (buf : & mut [u8]) { for (i , b) in buf . iter_mut () . enumerate () { * b = (i % 251) as u8 ; } }
    };
}

paint_test_input!();