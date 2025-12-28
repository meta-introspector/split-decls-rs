macro_rules! S_GB_ZEROFILL {
    () => {
        # [doc = " zero fill on demand section (that can be larger than 4 gigabytes)"] pub const S_GB_ZEROFILL : u32 = 0xc ;
    };
}

S_GB_ZEROFILL!();