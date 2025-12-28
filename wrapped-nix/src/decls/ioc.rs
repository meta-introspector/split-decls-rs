macro_rules! ioc {
    () => {
        # [macro_export] # [doc (hidden)] macro_rules ! ioc { ($ inout : expr , $ group : expr , $ num : expr , $ len : expr) => { $ inout | (($ len as $ crate :: sys :: ioctl :: ioctl_num_type & $ crate :: sys :: ioctl :: IOCPARM_MASK) << 16) | (($ group as $ crate :: sys :: ioctl :: ioctl_num_type) << 8) | ($ num as $ crate :: sys :: ioctl :: ioctl_num_type) } ; }
    };
}

ioc!()