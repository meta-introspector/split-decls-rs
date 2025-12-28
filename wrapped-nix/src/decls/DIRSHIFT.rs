macro_rules! DIRSHIFT {
    () => {
        # [doc (hidden)] pub const DIRSHIFT : ioctl_num_type = SIZESHIFT + SIZEBITS as ioctl_num_type ;
    };
}

DIRSHIFT!()