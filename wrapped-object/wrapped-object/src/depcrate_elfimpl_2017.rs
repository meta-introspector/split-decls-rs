// Generated macro for impl_2017 (impl)
macro_rules! Depcrate_elfimpl_2017 {
() => {
// Module: crate::elf
// Provides: {"impl_2017"}
// Dependencies: {}
impl < E : Endian > From < Rel32 < E > > for Rela32 < E > { fn from (rel : Rel32 < E >) -> Self { Rela32 { r_offset : rel . r_offset , r_info : rel . r_info , r_addend : I32 :: default () , } } }
};
}
