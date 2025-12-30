// Generated macro for impl_121 (impl)
macro_rules! Depcrate_pemimpl_121 {
() => {
// Module: crate::pem
// Provides: {"impl_121"}
// Dependencies: {}
impl < T : PemObjectFilter + From < Vec < u8 > > > PemObject for T { fn from_pem (kind : SectionKind , der : Vec < u8 >) -> Option < Self > { match Self :: KIND == kind { true => Some (Self :: from (der)) , false => None , } } }
};
}
