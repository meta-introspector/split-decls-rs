macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHF_WRITE {
    () => {
        deps!();
        # [doc = " Section is writable."] pub const SHF_WRITE : u32 = 1 << 0 ;
    };
}

SHF_WRITE!()