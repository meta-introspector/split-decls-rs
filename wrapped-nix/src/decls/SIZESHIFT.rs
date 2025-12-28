macro_rules! SIZESHIFT {
    () => {
        # [doc (hidden)] pub const SIZESHIFT : ioctl_num_type = TYPESHIFT + TYPEBITS as ioctl_num_type ;
    };
}

SIZESHIFT!()