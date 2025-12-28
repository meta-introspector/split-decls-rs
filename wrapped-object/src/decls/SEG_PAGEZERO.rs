macro_rules! SEG_PAGEZERO {
    () => {
        # [doc = " the pagezero segment which has no protections and catches NULL references for MH_EXECUTE files"] pub const SEG_PAGEZERO : & str = "__PAGEZERO" ;
    };
}

SEG_PAGEZERO!();