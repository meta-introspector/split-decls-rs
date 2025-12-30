// Generated macro for CoffBigSection (type)
macro_rules! Depcrate_read_coff_sectionCoffBigSection {
() => {
// Module: crate::read::coff::section
// Provides: {"CoffBigSection"}
// Dependencies: {}
# [doc = " A section in a [`CoffBigFile`](super::CoffBigFile)."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSection`] trait implementation."] pub type CoffBigSection < 'data , 'file , R = & 'data [u8] > = CoffSection < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
};
}
