macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_80 {
    () => {
        deps!();
        dw ! (# [doc = " The encodings of the constants used in the `DW_AT_calling_convention` attribute."] # [doc = ""] # [doc = " See Section 7.15, Table 7.19."] DwCc (u8) { DW_CC_normal = 0x01 , DW_CC_program = 0x02 , DW_CC_nocall = 0x03 , DW_CC_pass_by_reference = 0x04 , DW_CC_pass_by_value = 0x05 , DW_CC_lo_user = 0x40 , DW_CC_hi_user = 0xff , }) ;
    };
}

macro_80!()