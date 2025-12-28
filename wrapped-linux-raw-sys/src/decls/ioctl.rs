macro_rules! ioctl {
    () => {
        # [cfg (feature = "ioctl")] # [cfg (all (target_arch = "x86_64" , target_pointer_width = "32"))] # [path = "x32/ioctl.rs"] pub mod ioctl ;
    };
}

ioctl!();