macro_rules! DIRMASK {
    () => {
        # [doc (hidden)] pub const DIRMASK : ioctl_num_type = (1 << DIRBITS) - 1 ;
    };
}

DIRMASK!();