macro_rules! USIZE_ALIGN {
    () => {
        # [doc = " The bits that must be zero for a `*const usize` to be properly aligned."] const USIZE_ALIGN : usize = USIZE_BYTES - 1 ;
    };
}

USIZE_ALIGN!();