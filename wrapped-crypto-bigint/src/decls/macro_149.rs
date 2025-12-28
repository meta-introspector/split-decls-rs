macro_rules! macro_149 {
    () => {
        # [cfg (not (any (target_pointer_width = "32" , target_pointer_width = "64")))] compile_error ! ("this crate builds on 32-bit and 64-bit platforms only") ;
    };
}

macro_149!();