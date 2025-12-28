macro_rules! ioctl_num_type {
    () => {
        # [doc (hidden)] # [cfg (solarish)] pub type ioctl_num_type = :: libc :: c_int ;
    };
}

ioctl_num_type!();