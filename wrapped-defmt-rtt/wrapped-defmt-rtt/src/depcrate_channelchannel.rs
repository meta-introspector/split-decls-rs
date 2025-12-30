// Generated macro for Channel (struct)
macro_rules! Depcrate_channelChannel {
() => {
// Module: crate::channel
// Provides: {"Channel"}
// Dependencies: {}
# [doc = " RTT Up channel"] # [repr (C)] pub (crate) struct Channel { pub name : * const u8 , # [doc = " Pointer to the RTT buffer."] pub buffer : * mut u8 , pub size : usize , # [doc = " Written by the target."] pub write : AtomicUsize , # [doc = " Written by the host."] pub read : AtomicUsize , # [doc = " Channel properties."] # [doc = ""] # [doc = " Currently, only the lowest 2 bits are used to set the channel mode (see constants below)."] pub flags : AtomicUsize , }
};
}
