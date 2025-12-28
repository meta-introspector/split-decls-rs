macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_87 {
    () => {
        deps!();
        dw ! (# [doc = " The encodings for the extended opcodes for line number information."] # [doc = ""] # [doc = " See Section 7.22, Table 7.26."] DwLne (u8) { DW_LNE_end_sequence = 0x01 , DW_LNE_set_address = 0x02 , DW_LNE_define_file = 0x03 , DW_LNE_set_discriminator = 0x04 , DW_LNE_lo_user = 0x80 , DW_LNE_hi_user = 0xff , }) ;
    };
}

macro_87!()