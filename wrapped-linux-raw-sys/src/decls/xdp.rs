macro_rules! xdp {
    () => {
        # [cfg (feature = "xdp")] # [cfg (all (target_arch = "x86_64" , target_pointer_width = "32"))] # [path = "x32/xdp.rs"] pub mod xdp ;
    };
}

xdp!();