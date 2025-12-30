// Generated macro for UlpsEq (trait)
macro_rules! Depcrate_ulps_eqUlpsEq {
() => {
// Module: crate::ulps_eq
// Provides: {"UlpsEq"}
// Dependencies: {}
# [doc = " Equality comparisons between two numbers using both the absolute difference and ULPs"] # [doc = " (Units in Last Place) based comparisons."] pub trait UlpsEq < Rhs = Self > : AbsDiffEq < Rhs > where Rhs : ? Sized , { # [doc = " The default ULPs to tolerate when testing values that are far-apart."] # [doc = ""] # [doc = " This is used when no `max_ulps` value is supplied to the [`ulps_eq`] macro."] fn default_max_ulps () -> u32 ; # [doc = " A test for equality that uses units in the last place (ULP) if the values are far apart."] fn ulps_eq (& self , other : & Rhs , epsilon : Self :: Epsilon , max_ulps : u32) -> bool ; # [doc = " The inverse of [`UlpsEq::ulps_eq`]."] fn ulps_ne (& self , other : & Rhs , epsilon : Self :: Epsilon , max_ulps : u32) -> bool { ! Self :: ulps_eq (self , other , epsilon , max_ulps) } }
};
}
