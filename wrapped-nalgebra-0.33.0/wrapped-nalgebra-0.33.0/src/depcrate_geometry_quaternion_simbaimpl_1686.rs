// Generated macro for impl_1686 (impl)
macro_rules! Depcrate_geometry_quaternion_simbaimpl_1686 {
() => {
// Module: crate::geometry::quaternion_simba
// Provides: {"impl_1686"}
// Dependencies: {}
impl < T : Scalar + SimdValue > SimdValue for Quaternion < T > where T :: Element : Scalar , { const LANES : usize = T :: LANES ; type Element = Quaternion < T :: Element > ; type SimdBool = T :: SimdBool ; # [inline] fn splat (val : Self :: Element) -> Self { Vector4 :: splat (val . coords) . into () } # [inline] fn extract (& self , i : usize) -> Self :: Element { self . coords . extract (i) . into () } # [inline] unsafe fn extract_unchecked (& self , i : usize) -> Self :: Element { self . coords . extract_unchecked (i) . into () } # [inline] fn replace (& mut self , i : usize , val : Self :: Element) { self . coords . replace (i , val . coords) } # [inline] unsafe fn replace_unchecked (& mut self , i : usize , val : Self :: Element) { self . coords . replace_unchecked (i , val . coords) } # [inline] fn select (self , cond : Self :: SimdBool , other : Self) -> Self { self . coords . select (cond , other . coords) . into () } }
};
}
