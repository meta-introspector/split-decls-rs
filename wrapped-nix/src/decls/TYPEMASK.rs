macro_rules! TYPEMASK {
    () => {
        # [doc (hidden)] pub const TYPEMASK : ioctl_num_type = (1 << TYPEBITS) - 1 ;
    };
}

TYPEMASK!()