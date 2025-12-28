macro_rules! LenUint {
    () => {
        # [cfg (target_pointer_width = "16")] pub (crate) type LenUint = u16 ;
    };
}

LenUint!();