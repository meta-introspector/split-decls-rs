// Generated macro for impl_232 (impl)
macro_rules! Depcrate_sliceimpl_232 {
() => {
// Module: crate::slice
// Provides: {"impl_232"}
// Dependencies: {}
impl HalfFloatSliceExt for [f16] { # [inline] fn reinterpret_cast (& self) -> & [u16] { transmute_ref ! (self) } # [inline] fn reinterpret_cast_mut (& mut self) -> & mut [u16] { transmute_mut ! (self) } # [inline] fn convert_from_f32_slice (& mut self , src : & [f32]) { assert_eq ! (self . len () , src . len () , "destination and source slices have different lengths") ; arch :: f32_to_f16_slice (src , self . reinterpret_cast_mut ()) } # [inline] fn convert_from_f64_slice (& mut self , src : & [f64]) { assert_eq ! (self . len () , src . len () , "destination and source slices have different lengths") ; arch :: f64_to_f16_slice (src , self . reinterpret_cast_mut ()) } # [inline] fn convert_to_f32_slice (& self , dst : & mut [f32]) { assert_eq ! (self . len () , dst . len () , "destination and source slices have different lengths") ; arch :: f16_to_f32_slice (self . reinterpret_cast () , dst) } # [inline] fn convert_to_f64_slice (& self , dst : & mut [f64]) { assert_eq ! (self . len () , dst . len () , "destination and source slices have different lengths") ; arch :: f16_to_f64_slice (self . reinterpret_cast () , dst) } # [cfg (any (feature = "alloc" , feature = "std"))] # [inline] # [allow (clippy :: uninit_vec)] fn to_f32_vec (& self) -> Vec < f32 > { let mut vec = vec ! [0f32 ; self . len ()] ; self . convert_to_f32_slice (& mut vec) ; vec } # [cfg (any (feature = "alloc" , feature = "std"))] # [inline] # [allow (clippy :: uninit_vec)] fn to_f64_vec (& self) -> Vec < f64 > { let mut vec = vec ! [0f64 ; self . len ()] ; self . convert_to_f64_slice (& mut vec) ; vec } }
};
}
