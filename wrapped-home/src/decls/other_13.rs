macro_rules! other_13 {
    () => {
        unsafe extern "C" { fn wcslen (buf : * const u16) -> usize ; }
    };
}

other_13!()