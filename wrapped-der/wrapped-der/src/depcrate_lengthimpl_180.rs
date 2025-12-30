// Generated macro for impl_180 (impl)
macro_rules! Depcrate_lengthimpl_180 {
() => {
// Module: crate::length
// Provides: {"impl_180"}
// Dependencies: {}
impl Add for Length { type Output = Result < Self > ; fn add (self , other : Self) -> Result < Self > { self . inner . checked_add (other . inner) . ok_or_else (| | ErrorKind :: Overflow . into ()) . map (Self :: new) } }
};
}
