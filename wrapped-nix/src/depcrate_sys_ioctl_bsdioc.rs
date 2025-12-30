// Generated macro for ioc (macro)
macro_rules! Depcrate_sys_ioctl_bsdioc {
() => {
// Module: crate::sys::ioctl::bsd
// Provides: {"ioc"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! ioc { ($ inout : expr , $ group : expr , $ num : expr , $ len : expr) => { $ inout | (($ len as $ crate :: sys :: ioctl :: ioctl_num_type & $ crate :: sys :: ioctl :: IOCPARM_MASK) << 16) | (($ group as $ crate :: sys :: ioctl :: ioctl_num_type) << 8) | ($ num as $ crate :: sys :: ioctl :: ioctl_num_type) } ; }
};
}
