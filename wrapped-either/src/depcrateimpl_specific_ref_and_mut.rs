// Generated macro for impl_specific_ref_and_mut (macro)
macro_rules! Depcrateimpl_specific_ref_and_mut {
() => {
// Module: crate
// Provides: {"impl_specific_ref_and_mut"}
// Dependencies: {}
macro_rules ! impl_specific_ref_and_mut { ($ t : ty , $ ($ attr : meta) ,*) => { $ (# [$ attr]) * impl < L , R > AsRef <$ t > for Either < L , R > where L : AsRef <$ t >, R : AsRef <$ t > { fn as_ref (& self) -> &$ t { for_both ! (self , inner => inner . as_ref ()) } } $ (# [$ attr]) * impl < L , R > AsMut <$ t > for Either < L , R > where L : AsMut <$ t >, R : AsMut <$ t > { fn as_mut (& mut self) -> & mut $ t { for_both ! (self , inner => inner . as_mut ()) } } } ; }
};
}
