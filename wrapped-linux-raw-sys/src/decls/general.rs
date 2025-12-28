macro_rules! general {
    () => {
        # [cfg (feature = "general")] # [cfg (all (target_arch = "x86_64" , target_pointer_width = "32"))] # [path = "x32/general.rs"] pub mod general ;
    };
}

general!()