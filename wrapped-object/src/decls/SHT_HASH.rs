macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! SHT_HASH {
    () => {
        deps!();
        # [doc = " Symbol hash table."] pub const SHT_HASH : u32 = 5 ;
    };
}

SHT_HASH!()