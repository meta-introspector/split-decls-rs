// Generated macro for impl_2020 (impl)
macro_rules! Depcrate_elfimpl_2020 {
() => {
// Module: crate::elf
// Provides: {"impl_2020"}
// Dependencies: {}
impl < E : Endian > From < Rel64 < E > > for Rela64 < E > { fn from (rel : Rel64 < E >) -> Self { Rela64 { r_offset : rel . r_offset , r_info : rel . r_info , r_addend : I64 :: default () , } } }
};
}
