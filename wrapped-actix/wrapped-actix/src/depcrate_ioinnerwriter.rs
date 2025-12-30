// Generated macro for InnerWriter (struct)
macro_rules! Depcrate_ioInnerWriter {
() => {
// Module: crate::io
// Provides: {"InnerWriter"}
// Dependencies: {}
struct InnerWriter < E : From < io :: Error > > { flags : Flags , buffer : BytesMut , error : Option < E > , low : usize , high : usize , handle : SpawnHandle , task : Option < task :: Waker > , }
};
}
