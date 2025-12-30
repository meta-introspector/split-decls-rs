// Generated macro for MapToCurve (trait)
macro_rules! Depcrate_map2curveMapToCurve {
() => {
// Module: crate::map2curve
// Provides: {"MapToCurve"}
// Dependencies: {}
# [doc = " Trait for converting field elements into a point via a mapping method like"] # [doc = " Simplified Shallue-van de Woestijne-Ulas or Elligator."] pub trait MapToCurve : CurveArithmetic < ProjectivePoint : CofactorGroup < Subgroup = Self :: ProjectivePoint > > { # [doc = " The target security level in bytes:"] # [doc = " <https://www.rfc-editor.org/rfc/rfc9380.html#section-8.9-2.2>"] # [doc = " <https://www.rfc-editor.org/rfc/rfc9380.html#name-target-security-levels>"] type SecurityLevel : Unsigned ; # [doc = " The field element representation for a group value with multiple elements."] type FieldElement : Reduce < Array < u8 , Self :: Length > > + Default + Copy ; # [doc = " The `L` parameter as specified in the [RFC](https://www.rfc-editor.org/rfc/rfc9380.html#section-5-6)."] type Length : ArraySize + NonZero ; # [doc = " Map a field element into a curve point."] fn map_to_curve (element : Self :: FieldElement) -> ProjectivePoint < Self > ; }
};
}
