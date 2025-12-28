macro_rules! STYP_PAD {
    () => {
        # [doc = " Specifies a pad section. A section of this type is used to provide alignment"] # [doc = " padding between sections within an XCOFF executable object file. This section"] # [doc = " header type is obsolete since padding is allowed in an XCOFF file without a"] # [doc = " corresponding pad section header."] pub const STYP_PAD : u16 = 0x08 ;
    };
}

STYP_PAD!()