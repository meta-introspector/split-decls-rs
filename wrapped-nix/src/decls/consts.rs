macro_rules! consts {
    () => {
        mod consts { use crate :: sys :: ioctl :: ioctl_num_type ; # [doc (hidden)] pub const VOID : ioctl_num_type = 0x2000_0000 ; # [doc (hidden)] pub const OUT : ioctl_num_type = 0x4000_0000 ; # [doc (hidden)] # [allow (overflowing_literals)] pub const IN : ioctl_num_type = 0x8000_0000 ; # [doc (hidden)] pub const INOUT : ioctl_num_type = IN | OUT ; # [doc (hidden)] pub const IOCPARM_MASK : ioctl_num_type = 0x1fff ; }
    };
}

consts!();