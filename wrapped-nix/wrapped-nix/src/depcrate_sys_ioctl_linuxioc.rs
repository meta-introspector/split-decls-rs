// Generated macro for ioc (macro)
macro_rules! Depcrate_sys_ioctl_linuxioc {
() => {
// Module: crate::sys::ioctl::linux
// Provides: {"ioc"}
// Dependencies: {}
# [doc = " Encode an ioctl command."] # [macro_export] # [doc (hidden)] macro_rules ! ioc { ($ dir : expr , $ ty : expr , $ nr : expr , $ sz : expr) => { (($ dir as $ crate :: sys :: ioctl :: ioctl_num_type & $ crate :: sys :: ioctl :: DIRMASK) << $ crate :: sys :: ioctl :: DIRSHIFT) | (($ ty as $ crate :: sys :: ioctl :: ioctl_num_type & $ crate :: sys :: ioctl :: TYPEMASK) << $ crate :: sys :: ioctl :: TYPESHIFT) | (($ nr as $ crate :: sys :: ioctl :: ioctl_num_type & $ crate :: sys :: ioctl :: NRMASK) << $ crate :: sys :: ioctl :: NRSHIFT) | (($ sz as $ crate :: sys :: ioctl :: ioctl_num_type & $ crate :: sys :: ioctl :: SIZEMASK) << $ crate :: sys :: ioctl :: SIZESHIFT) } ; }
};
}
