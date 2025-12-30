// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'a , BlockSize : ArraySize > PaddedData < 'a , BlockSize > { # [doc = " Unwrap the `Pad` variant."] pub fn unwrap (self) -> (& 'a [Array < u8 , BlockSize >] , Array < u8 , BlockSize >) { match self { PaddedData :: Pad { blocks , tail_block } => (blocks , tail_block) , PaddedData :: NoPad { .. } => { panic ! ("Expected `PaddedData::Pad`, but got `PaddedData::NoPad`") ; } PaddedData :: Error => { panic ! ("Expected `PaddedData::Pad`, but got `PaddedData::Error`") ; } } } }
};
}
