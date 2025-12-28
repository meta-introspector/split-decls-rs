macro_rules! BUFFER_SIZE {
    () => {
        const BUFFER_SIZE : usize = BUFFER_CAPACITY * ELEM_SIZE ;
    };
}

BUFFER_SIZE!();