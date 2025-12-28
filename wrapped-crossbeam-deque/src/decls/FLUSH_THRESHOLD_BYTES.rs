macro_rules! FLUSH_THRESHOLD_BYTES {
    () => {
        const FLUSH_THRESHOLD_BYTES : usize = 1 << 10 ;
    };
}

FLUSH_THRESHOLD_BYTES!()