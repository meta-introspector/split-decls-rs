macro_rules! system {
    () => {
        # [cfg (feature = "system")] # [cfg (all (target_arch = "x86_64" , target_pointer_width = "32"))] # [path = "x32/system.rs"] pub mod system ;
    };
}

system!();