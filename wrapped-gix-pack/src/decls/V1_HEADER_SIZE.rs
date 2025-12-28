macro_rules! V1_HEADER_SIZE {
    () => {
        const V1_HEADER_SIZE : usize = FAN_LEN * N32_SIZE ;
    };
}

V1_HEADER_SIZE!();