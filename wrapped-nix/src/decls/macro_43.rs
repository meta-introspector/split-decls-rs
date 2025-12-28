macro_rules! macro_43 {
    () => {
        # [cfg (linux_android)] # [cfg (feature = "zerocopy")] libc_bitflags ! { # [doc = " Additional flags to `splice` and friends."] # [cfg_attr (docsrs , doc (cfg (feature = "zerocopy")))] pub struct SpliceFFlags : c_uint { # [doc = " Request that pages be moved instead of copied."] # [doc = ""] # [doc = " Not applicable to `vmsplice`."] SPLICE_F_MOVE ; # [doc = " Do not block on I/O."] SPLICE_F_NONBLOCK ; # [doc = " Hint that more data will be coming in a subsequent splice."] # [doc = ""] # [doc = " Not applicable to `vmsplice`."] SPLICE_F_MORE ; # [doc = " Gift the user pages to the kernel."] # [doc = ""] # [doc = " Not applicable to `splice`."] SPLICE_F_GIFT ; } }
    };
}

macro_43!();