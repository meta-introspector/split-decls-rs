macro_rules! SEG_LINKINFO {
    () => {
        # [doc = " the segment overlapping with linkedit containing linking information"] pub const SEG_LINKINFO : & str = "__LINKINFO" ;
    };
}

SEG_LINKINFO!();