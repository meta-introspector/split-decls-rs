macro_rules! V2_HEADER_SIZE {
    () => {
        const V2_HEADER_SIZE : usize = N32_SIZE * 2 + FAN_LEN * N32_SIZE ;
    };
}

V2_HEADER_SIZE!();