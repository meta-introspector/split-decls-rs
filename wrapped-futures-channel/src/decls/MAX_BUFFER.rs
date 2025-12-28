macro_rules! MAX_BUFFER {
    () => {
        const MAX_BUFFER : usize = MAX_CAPACITY >> 1 ;
    };
}

MAX_BUFFER!();