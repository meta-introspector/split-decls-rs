// Generated macro for data (function)
macro_rules! Depcrate_framesdata {
() => {
// Module: crate::frames
// Provides: {"data"}
// Dependencies: {}
pub fn data < T , B > (id : T , buf : B) -> Mock < frame :: Data > where T : Into < StreamId > , B : AsRef < [u8] > , { let buf = Bytes :: copy_from_slice (buf . as_ref ()) ; Mock (frame :: Data :: new (id . into () , buf)) }
};
}
