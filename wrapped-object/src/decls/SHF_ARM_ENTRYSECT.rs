macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHF_ARM_ENTRYSECT {
    () => {
        deps!();
        # [doc = " Section contains an entry point"] pub const SHF_ARM_ENTRYSECT : u32 = 0x1000_0000 ;
    };
}

SHF_ARM_ENTRYSECT!();