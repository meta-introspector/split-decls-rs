// Generated macro for impl_222 (impl)
macro_rules! Depcrate_x86_64_sse2impl_222 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_222"}
// Dependencies: {}
impl < S3 , S4 , NI > Debug for u32x4_sse2 < S3 , S4 , NI > where Self : Copy + MultiLane < [u32 ; 4] > , { # [cold] fn fmt (& self , fmt : & mut Formatter) -> Result { fmt . write_fmt (format_args ! ("{:08x?}" , & self . to_lanes ())) } }
};
}
