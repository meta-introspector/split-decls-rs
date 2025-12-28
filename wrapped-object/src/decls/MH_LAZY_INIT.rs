macro_rules! MH_LAZY_INIT {
    () => {
        # [doc = " the shared library init routine is to be run lazily via catching memory faults to its writeable segments (obsolete)"] pub const MH_LAZY_INIT : u32 = 0x40 ;
    };
}

MH_LAZY_INIT!();