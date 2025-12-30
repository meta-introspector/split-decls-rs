// Generated macro for convert_ioctl_res (macro)
macro_rules! Depcrate_sys_ioctlconvert_ioctl_res {
() => {
// Module: crate::sys::ioctl
// Provides: {"convert_ioctl_res"}
// Dependencies: {}
# [doc = " Convert raw ioctl return value to a Nix result"] # [macro_export] # [doc (hidden)] macro_rules ! convert_ioctl_res { ($ w : expr) => { { $ crate :: errno :: Errno :: result ($ w) } } ; }
};
}
