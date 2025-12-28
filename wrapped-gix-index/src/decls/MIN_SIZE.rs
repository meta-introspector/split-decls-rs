macro_rules! MIN_SIZE {
    () => {
        # [doc = " The minimal size of the extension, depending on the shortest hash."] pub const MIN_SIZE : usize = 4 + gix_hash :: Kind :: shortest () . len_in_bytes () ;
    };
}

MIN_SIZE!();