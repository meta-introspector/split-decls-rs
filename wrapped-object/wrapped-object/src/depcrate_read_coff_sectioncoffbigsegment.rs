// Generated macro for CoffBigSegment (type)
macro_rules! Depcrate_read_coff_sectionCoffBigSegment {
() => {
// Module: crate::read::coff::section
// Provides: {"CoffBigSegment"}
// Dependencies: {}
# [doc = " A loadable section in a [`CoffBigFile`](super::CoffBigFile)."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSegment`] trait implementation."] pub type CoffBigSegment < 'data , 'file , R = & 'data [u8] > = CoffSegment < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
};
}
