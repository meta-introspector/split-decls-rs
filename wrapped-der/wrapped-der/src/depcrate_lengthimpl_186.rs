// Generated macro for impl_186 (impl)
macro_rules! Depcrate_lengthimpl_186 {
() => {
// Module: crate::length
// Provides: {"impl_186"}
// Dependencies: {}
impl Sub for Length { type Output = Result < Self > ; fn sub (self , other : Length) -> Result < Self > { self . inner . checked_sub (other . inner) . ok_or_else (| | ErrorKind :: Overflow . into ()) . map (Self :: new) } }
};
}
