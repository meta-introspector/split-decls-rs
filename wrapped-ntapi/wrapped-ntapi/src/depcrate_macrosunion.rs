// Generated macro for UNION (macro)
macro_rules! Depcrate_macrosUNION {
() => {
// Module: crate::macros
// Provides: {"UNION"}
// Dependencies: {}
macro_rules ! UNION { ($ (# [$ attrs : meta]) * union $ name : ident { $ ($ variant : ident : $ ftype : ty ,) + }) => (# [repr (C)] $ (# [$ attrs]) * pub union $ name { $ (pub $ variant : $ ftype ,) + } impl Copy for $ name { } impl Clone for $ name { # [inline] fn clone (& self) -> $ name { * self } } # [cfg (feature = "impl-default")] impl Default for $ name { # [inline] fn default () -> $ name { unsafe { $ crate :: _core :: mem :: zeroed () } } }) ; }
};
}
