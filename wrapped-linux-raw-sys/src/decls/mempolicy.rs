macro_rules! mempolicy {
    () => {
        # [cfg (feature = "mempolicy")] # [cfg (all (target_arch = "x86_64" , target_pointer_width = "32"))] # [path = "x32/mempolicy.rs"] pub mod mempolicy ;
    };
}

mempolicy!()