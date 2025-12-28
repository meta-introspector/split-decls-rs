macro_rules! errno {
    () => {
        # [cfg (feature = "errno")] # [cfg (all (target_arch = "x86_64" , target_pointer_width = "32"))] # [path = "x32/errno.rs"] pub mod errno ;
    };
}

errno!()