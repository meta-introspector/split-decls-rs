macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHF_LINK_ORDER {
    () => {
        deps!();
        # [doc = " Section has special ordering requirements when combining sections."] pub const SHF_LINK_ORDER : u32 = 1 << 7 ;
    };
}

SHF_LINK_ORDER!()