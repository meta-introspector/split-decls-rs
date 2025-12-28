macro_rules! MIN_MEMORY {
    () => {
        # [doc = " The minimum amount of memory."] pub (crate) const MIN_MEMORY : u32 = 8 * LANES ;
    };
}

MIN_MEMORY!()