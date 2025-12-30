// Generated macro for macro_232 (macro)
macro_rules! Depcrate_typesmacro_232 {
() => {
// Module: crate::types
// Provides: {"macro_232"}
// Dependencies: {}
bitflags ! { # [doc = " Options for [`Timeout`](super::Timeout)."] # [doc = ""] # [doc = " The default behavior is to treat the timespec as a relative time interval. `flags` may"] # [doc = " contain [`TimeoutFlags::ABS`] to indicate the timespec represents an absolute"] # [doc = " time. When an absolute time is being specified, the kernel will use its monotonic clock"] # [doc = " unless one of the following flags is set (they may not both be set):"] # [doc = " [`TimeoutFlags::BOOTTIME`] or [`TimeoutFlags::REALTIME`]."] # [doc = ""] # [doc = " The default behavior when the timeout expires is to sever dependent links, as a failed"] # [doc = " request normally would. To keep the links untouched include [`TimeoutFlags::ETIME_SUCCESS`]."] # [doc = " CQE will still contain -libc::ETIME in the res field"] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] pub struct TimeoutFlags : u32 { const ABS = sys :: IORING_TIMEOUT_ABS ; const BOOTTIME = sys :: IORING_TIMEOUT_BOOTTIME ; const REALTIME = sys :: IORING_TIMEOUT_REALTIME ; const LINK_TIMEOUT_UPDATE = sys :: IORING_LINK_TIMEOUT_UPDATE ; const ETIME_SUCCESS = sys :: IORING_TIMEOUT_ETIME_SUCCESS ; const MULTISHOT = sys :: IORING_TIMEOUT_MULTISHOT ; } }
};
}
