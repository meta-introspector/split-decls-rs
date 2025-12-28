macro_rules! MAX_LINE_LEN {
    () => {
        const MAX_LINE_LEN : usize = MAX_DATA_LEN + U16_HEX_BYTES ;
    };
}

MAX_LINE_LEN!();