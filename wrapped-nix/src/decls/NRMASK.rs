macro_rules! NRMASK {
    () => {
        # [doc (hidden)] pub const NRMASK : ioctl_num_type = (1 << NRBITS) - 1 ;
    };
}

NRMASK!();