macro_rules! bootparam {
    () => {
        # [cfg (feature = "bootparam")] # [cfg (all (target_arch = "x86_64" , target_pointer_width = "32"))] # [path = "x32/bootparam.rs"] pub mod bootparam ;
    };
}

bootparam!()