macro_rules! macro_0 {
    () => {
        # [cfg (not (any (target_pointer_width = "16" , target_pointer_width = "32" , target_pointer_width = "64")))] compile_error ! ("memchr currently not supported on non-{16,32,64}") ;
    };
}

macro_0!()