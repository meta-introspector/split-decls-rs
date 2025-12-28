macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHF_ARM_COMDEF {
    () => {
        deps!();
        # [doc = " Section may be multiply defined in the input to a link step."] pub const SHF_ARM_COMDEF : u32 = 0x8000_0000 ;
    };
}

SHF_ARM_COMDEF!()