// Generated macro for Hi64 (trait)
macro_rules! Depcrate_lexical_mathHi64 {
() => {
// Module: crate::lexical::math
// Provides: {"Hi64"}
// Dependencies: {}
# [doc = " Trait to export the high 64-bits from a little-endian slice."] trait Hi64 < T > : AsRef < [T] > { # [doc = " Get the hi64 bits from a 1-limb slice."] fn hi64_1 (& self) -> (u64 , bool) ; # [doc = " Get the hi64 bits from a 2-limb slice."] fn hi64_2 (& self) -> (u64 , bool) ; # [doc = " Get the hi64 bits from a 3-limb slice."] fn hi64_3 (& self) -> (u64 , bool) ; # [doc = " High-level exporter to extract the high 64 bits from a little-endian slice."] # [inline] fn hi64 (& self) -> (u64 , bool) { match self . as_ref () . len () { 0 => (0 , false) , 1 => self . hi64_1 () , 2 => self . hi64_2 () , _ => self . hi64_3 () , } } }
};
}
