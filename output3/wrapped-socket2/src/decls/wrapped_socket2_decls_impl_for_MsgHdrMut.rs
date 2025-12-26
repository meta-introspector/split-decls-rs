use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(target_os = "redox"))]
impl<'addr, 'bufs, 'control> MsgHdrMut<'addr, 'bufs, 'control> {
    /// Create a new `MsgHdrMut` with all empty/zero fields.
    #[allow(clippy::new_without_default)]
    pub fn new() -> MsgHdrMut<'addr, 'bufs, 'control> {
        MsgHdrMut {
            inner: unsafe { mem::zeroed() },
            _lifetimes: PhantomData,
        }
    }
    /// Set the mutable address (name) of the message.
    ///
    /// Corresponds to setting `msg_name` and `msg_namelen` on Unix and `name`
    /// and `namelen` on Windows.
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn with_addr(mut self, addr: &'addr mut SockAddr) -> Self {
        sys::set_msghdr_name(&mut self.inner, addr);
        self
    }
    /// Set the mutable buffer(s) of the message.
    ///
    /// Corresponds to setting `msg_iov` and `msg_iovlen` on Unix and `lpBuffers`
    /// and `dwBufferCount` on Windows.
    pub fn with_buffers(mut self, bufs: &'bufs mut [MaybeUninitSlice<'_>]) -> Self {
        sys::set_msghdr_iov(&mut self.inner, bufs.as_mut_ptr().cast(), bufs.len());
        self
    }
    /// Set the mutable control buffer of the message.
    ///
    /// Corresponds to setting `msg_control` and `msg_controllen` on Unix and
    /// `Control` on Windows.
    pub fn with_control(mut self, buf: &'control mut [MaybeUninit<u8>]) -> Self {
        sys::set_msghdr_control(&mut self.inner, buf.as_mut_ptr().cast(), buf.len());
        self
    }
    /// Returns the flags of the message.
    pub fn flags(&self) -> RecvFlags {
        sys::msghdr_flags(&self.inner)
    }
    /// Gets the length of the control buffer.
    ///
    /// Can be used to determine how much, if any, of the control buffer was filled by `recvmsg`.
    ///
    /// Corresponds to `msg_controllen` on Unix and `Control.len` on Windows.
    pub fn control_len(&self) -> usize {
        sys::msghdr_control_len(&self.inner)
    }
}
