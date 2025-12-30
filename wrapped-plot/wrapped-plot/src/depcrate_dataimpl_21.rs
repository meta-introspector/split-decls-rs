// Generated macro for impl_21 (impl)
macro_rules! Depcrate_dataimpl_21 {
() => {
// Module: crate::data
// Provides: {"impl_21"}
// Dependencies: {}
impl < A , B > Row for (A , B) where A : Data , B : Data , { type Scale = (f64 , f64) ; fn append_to (self , buffer : & mut Vec < u8 > , scale : (f64 , f64)) { let (a , b) = self ; write_f64 (buffer , a . f64 () * scale . 0) . unwrap () ; write_f64 (buffer , b . f64 () * scale . 1) . unwrap () ; } fn ncols () -> usize { 2 } }
};
}
