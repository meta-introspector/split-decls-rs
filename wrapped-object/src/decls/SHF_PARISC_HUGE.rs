macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHF_PARISC_HUGE {
    () => {
        deps!();
        # [doc = " Section far from gp."] pub const SHF_PARISC_HUGE : u32 = 0x4000_0000 ;
    };
}

SHF_PARISC_HUGE!()