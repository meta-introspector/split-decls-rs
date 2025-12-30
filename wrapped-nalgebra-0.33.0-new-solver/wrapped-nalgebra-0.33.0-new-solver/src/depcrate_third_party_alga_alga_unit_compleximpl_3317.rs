// Generated macro for impl_3317 (impl)
macro_rules! Depcrate_third_party_alga_alga_unit_compleximpl_3317 {
() => {
// Module: crate::third_party::alga::alga_unit_complex
// Provides: {"impl_3317"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > Rotation < Point2 < T > > for UnitComplex < T > { # [inline] fn powf (& self , n : T) -> Option < Self > { Some (self . powf (n)) } # [inline] fn rotation_between (a : & Vector2 < T > , b : & Vector2 < T >) -> Option < Self > { Some (Self :: rotation_between (a , b)) } # [inline] fn scaled_rotation_between (a : & Vector2 < T > , b : & Vector2 < T > , s : T) -> Option < Self > { Some (Self :: scaled_rotation_between (a , b , s)) } }
};
}
