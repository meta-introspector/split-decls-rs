macro_rules! USIZE_BYTES {
    () => {
        const USIZE_BYTES : usize = core :: mem :: size_of :: < usize > () ;
    };
}

USIZE_BYTES!();