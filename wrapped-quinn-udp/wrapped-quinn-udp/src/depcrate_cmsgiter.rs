// Generated macro for Iter (struct)
macro_rules! Depcrate_cmsgIter {
() => {
// Module: crate::cmsg
// Provides: {"Iter"}
// Dependencies: {}
pub (crate) struct Iter < 'a , M : MsgHdr > { hdr : & 'a M , cmsg : Option < & 'a M :: ControlMessage > , }
};
}
