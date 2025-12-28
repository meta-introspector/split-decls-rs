macro_rules! TYPESHIFT {
    () => {
        # [doc (hidden)] pub const TYPESHIFT : ioctl_num_type = NRSHIFT + NRBITS as ioctl_num_type ;
    };
}

TYPESHIFT!();