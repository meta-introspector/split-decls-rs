// Generated macro for impl_1687 (impl)
macro_rules! Depcrate_geometry_quaternion_simbaimpl_1687 {
() => {
// Module: crate::geometry::quaternion_simba
// Provides: {"impl_1687"}
// Dependencies: {}
impl < T : Scalar + SimdValue > SimdValue for UnitQuaternion < T > where T :: Element : Scalar , { const LANES : usize = T :: LANES ; type Element = UnitQuaternion < T :: Element > ; type SimdBool = T :: SimdBool ; # [inline] fn splat (val : Self :: Element) -> Self { UnitQuaternion :: new_unchecked (Quaternion :: splat (val . into_inner ())) } # [inline] fn extract (& self , i : usize) -> Self :: Element { UnitQuaternion :: new_unchecked (self . as_ref () . extract (i)) } # [inline] unsafe fn extract_unchecked (& self , i : usize) -> Self :: Element { UnitQuaternion :: new_unchecked (self . as_ref () . extract_unchecked (i)) } # [inline] fn replace (& mut self , i : usize , val : Self :: Element) { self . as_mut_unchecked () . replace (i , val . into_inner ()) } # [inline] unsafe fn replace_unchecked (& mut self , i : usize , val : Self :: Element) { self . as_mut_unchecked () . replace_unchecked (i , val . into_inner ()) } # [inline] fn select (self , cond : Self :: SimdBool , other : Self) -> Self { UnitQuaternion :: new_unchecked (self . into_inner () . select (cond , other . into_inner ())) } }
};
}
