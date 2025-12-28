macro_rules! SIZEMASK {
    () => {
        # [doc (hidden)] pub const SIZEMASK : ioctl_num_type = (1 << SIZEBITS) - 1 ;
    };
}

SIZEMASK!();