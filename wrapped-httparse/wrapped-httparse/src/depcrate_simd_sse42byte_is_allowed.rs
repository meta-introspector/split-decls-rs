// Generated macro for byte_is_allowed (function)
macro_rules! Depcrate_simd_sse42byte_is_allowed {
() => {
// Module: crate::simd::sse42
// Provides: {"byte_is_allowed"}
// Dependencies: {}
# [allow (clippy :: missing_safety_doc)] # [cfg (test)] unsafe fn byte_is_allowed (byte : u8 , f : unsafe fn (bytes : & mut Bytes < '_ >)) -> bool { let slice = [b'_' , b'_' , b'_' , b'_' , b'_' , b'_' , b'_' , b'_' , b'_' , b'_' , byte , b'_' , b'_' , b'_' , b'_' , b'_' ,] ; let mut bytes = Bytes :: new (& slice) ; f (& mut bytes) ; match bytes . pos () { 16 => true , 10 => false , _ => unreachable ! () , } }
};
}
