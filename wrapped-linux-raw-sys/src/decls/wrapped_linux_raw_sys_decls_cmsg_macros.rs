use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "net")]
pub mod cmsg_macros {
    use crate::ctypes::{c_long, c_uchar, c_uint};
    use crate::net::{cmsghdr, msghdr};
    use core::mem::size_of;
    use core::ptr;
    pub const unsafe fn CMSG_ALIGN(len: c_uint) -> c_uint {
        let c_long_size = size_of::<c_long>() as c_uint;
        (len + c_long_size - 1) & !(c_long_size - 1)
    }
    pub const unsafe fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut c_uchar {
        (cmsg as *mut c_uchar).add(size_of::<cmsghdr>())
    }
    pub const unsafe fn CMSG_SPACE(len: c_uint) -> c_uint {
        size_of::<cmsghdr>() as c_uint + CMSG_ALIGN(len)
    }
    pub const unsafe fn CMSG_LEN(len: c_uint) -> c_uint {
        size_of::<cmsghdr>() as c_uint + len
    }
    pub const unsafe fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
        if (*mhdr).msg_controllen < size_of::<cmsghdr>() as _ {
            return ptr::null_mut();
        }
        (*mhdr).msg_control as *mut cmsghdr
    }
    pub unsafe fn CMSG_NXTHDR(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr {
        let cmsg_len = (*cmsg).cmsg_len;
        let next_cmsg = (cmsg as *mut u8).add(CMSG_ALIGN(cmsg_len as _) as usize) as *mut cmsghdr;
        let max = ((*mhdr).msg_control as usize) + ((*mhdr).msg_controllen as usize);
        if cmsg_len < size_of::<cmsghdr>() as _ {
            return ptr::null_mut();
        }
        if next_cmsg.add(1) as usize > max
            || next_cmsg as usize + CMSG_ALIGN((*next_cmsg).cmsg_len as _) as usize > max
        {
            return ptr::null_mut();
        }
        next_cmsg
    }
}
