macro_rules! deps {
    () => {
        Note!();
    };
}

macro_rules! NT_VMCOREDD {
    () => {
        deps!();
        # [doc = " Vmcore Device Dump Note."] pub const NT_VMCOREDD : u32 = 0x700 ;
    };
}

NT_VMCOREDD!()