// Generated macro for impl_225 (impl)
macro_rules! Depcrate_x86_64_sse2impl_225 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_225"}
// Dependencies: {}
impl < S3 , S4 , NI > Debug for u64x4_sse2 < S3 , S4 , NI > where u64x2_sse2 < S3 , S4 , NI > : Copy + MultiLane < [u64 ; 2] > , { # [cold] fn fmt (& self , fmt : & mut Formatter) -> Result { let (a , b) = (self . 0 [0] . to_lanes () , self . 0 [1] . to_lanes ()) ; fmt . write_fmt (format_args ! ("{:016x?}" , & [a [0] , a [1] , b [0] , b [1]])) } }
};
}
