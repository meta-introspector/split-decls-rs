// Generated macro for impl_184 (impl)
macro_rules! Depcrate_lengthimpl_184 {
() => {
// Module: crate::length
// Provides: {"impl_184"}
// Dependencies: {}
impl Add < usize > for Length { type Output = Result < Self > ; fn add (self , other : usize) -> Result < Self > { self + Length :: try_from (other) ? } }
};
}
