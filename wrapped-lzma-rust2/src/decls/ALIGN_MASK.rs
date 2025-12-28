macro_rules! ALIGN_MASK {
    () => {
        const ALIGN_MASK : usize = ALIGN_SIZE - 1 ;
    };
}

ALIGN_MASK!()