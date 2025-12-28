macro_rules! STYP_OVRFLO {
    () => {
        # [doc = " Specifies a relocation or line-number field overflow section. A section"] # [doc = " header of this type contains the count of relocation entries and line"] # [doc = " number entries for some other section. This section header is required"] # [doc = " when either of the counts exceeds 65,534."] pub const STYP_OVRFLO : u16 = 0x8000 ;
    };
}

STYP_OVRFLO!()