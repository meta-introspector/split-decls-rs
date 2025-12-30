// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
impl < T > Index < IdxRange < T > > for Arena < T > { type Output = [T] ; fn index (& self , range : IdxRange < T >) -> & [T] { let start = range . range . start as usize ; let end = range . range . end as usize ; & self . data [start .. end] } }
};
}
