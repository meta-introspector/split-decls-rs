// Generated macro for impl_23 (impl)
macro_rules! Depcrate_dataimpl_23 {
() => {
// Module: crate::data
// Provides: {"impl_23"}
// Dependencies: {}
impl < A , B , C , D > Row for (A , B , C , D) where A : Data , B : Data , C : Data , D : Data , { type Scale = (f64 , f64 , f64 , f64) ; fn append_to (self , buffer : & mut Vec < u8 > , scale : (f64 , f64 , f64 , f64)) { let (a , b , c , d) = self ; write_f64 (buffer , a . f64 () * scale . 0) . unwrap () ; write_f64 (buffer , b . f64 () * scale . 1) . unwrap () ; write_f64 (buffer , c . f64 () * scale . 2) . unwrap () ; write_f64 (buffer , d . f64 () * scale . 3) . unwrap () ; } fn ncols () -> usize { 4 } }
};
}
