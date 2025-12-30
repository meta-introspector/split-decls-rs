// Generated macro for impl_170 (impl)
macro_rules! Depcrate_x86_64_sse2impl_170 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_170"}
// Dependencies: {}
impl < S3 , NI > Vec4 < u32 > for u32x4_sse2 < S3 , YesS4 , NI > where Self : MultiLane < [u32 ; 4] > , { # [inline (always)] fn extract (self , i : u32) -> u32 { self . to_lanes () [i as usize] } # [inline (always)] fn insert (self , v : u32 , i : u32) -> Self { Self :: new (unsafe { match i { 0 => _mm_insert_epi32 (self . x , v as i32 , 0) , 1 => _mm_insert_epi32 (self . x , v as i32 , 1) , 2 => _mm_insert_epi32 (self . x , v as i32 , 2) , 3 => _mm_insert_epi32 (self . x , v as i32 , 3) , _ => unreachable ! () , } }) } }
};
}
