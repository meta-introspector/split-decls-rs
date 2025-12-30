// Generated macro for impl_224 (impl)
macro_rules! Depcrate_x86_64_sse2impl_224 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_224"}
// Dependencies: {}
impl < S3 , S4 , NI > Debug for u64x2_sse2 < S3 , S4 , NI > where Self : Copy + MultiLane < [u64 ; 2] > , { # [cold] fn fmt (& self , fmt : & mut Formatter) -> Result { fmt . write_fmt (format_args ! ("{:016x?}" , & self . to_lanes ())) } }
};
}
