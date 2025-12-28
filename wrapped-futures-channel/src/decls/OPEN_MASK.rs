macro_rules! OPEN_MASK {
    () => {
        const OPEN_MASK : usize = usize :: MAX - (usize :: MAX >> 1) ;
    };
}

OPEN_MASK!()