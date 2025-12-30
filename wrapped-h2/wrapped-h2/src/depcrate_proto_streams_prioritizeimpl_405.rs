// Generated macro for impl_405 (impl)
macro_rules! Depcrate_proto_streams_prioritizeimpl_405 {
() => {
// Module: crate::proto::streams::prioritize
// Provides: {"impl_405"}
// Dependencies: {}
impl < B > Buf for Prioritized < B > where B : Buf , { fn remaining (& self) -> usize { self . inner . remaining () } fn chunk (& self) -> & [u8] { self . inner . chunk () } fn chunks_vectored < 'a > (& 'a self , dst : & mut [std :: io :: IoSlice < 'a >]) -> usize { self . inner . chunks_vectored (dst) } fn advance (& mut self , cnt : usize) { self . inner . advance (cnt) } }
};
}
