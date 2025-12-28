macro_rules! ALIGN_SIZE {
    () => {
        const ALIGN_SIZE : usize = 1 << ALIGN_BITS ;
    };
}

ALIGN_SIZE!();