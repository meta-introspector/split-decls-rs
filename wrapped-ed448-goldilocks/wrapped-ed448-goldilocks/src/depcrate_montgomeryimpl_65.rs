// Generated macro for impl_65 (impl)
macro_rules! Depcrate_montgomeryimpl_65 {
() => {
// Module: crate::montgomery
// Provides: {"impl_65"}
// Dependencies: {}
impl MontgomeryPoint { # [doc = " Returns the generator specified in RFC7748"] pub const GENERATOR : Self = Self ([0x05 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,]) ; # [doc = " Convert this point to an [`EdwardsPoint`]"] pub fn to_edwards (& self , _sign : u8) -> Option < EdwardsPoint > { todo ! () } # [doc = " Returns true if the point is one of the low order points"] pub fn is_low_order (& self) -> bool { (* self == Self :: LOW_A) || (* self == Self :: LOW_B) || (* self == Self :: LOW_C) } # [doc = " View the point as a byte slice"] pub fn as_bytes (& self) -> & [u8 ; 56] { & self . 0 } # [doc = " Convert the point to a ProjectiveMontgomeryPoint"] pub fn to_projective (& self) -> ProjectiveMontgomeryPoint { ProjectiveMontgomeryPoint { U : FieldElement :: from_bytes (& self . 0) , W : FieldElement :: ONE , } } }
};
}
