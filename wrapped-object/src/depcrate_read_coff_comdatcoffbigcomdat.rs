// Generated macro for CoffBigComdat (type)
macro_rules! Depcrate_read_coff_comdatCoffBigComdat {
() => {
// Module: crate::read::coff::comdat
// Provides: {"CoffBigComdat"}
// Dependencies: {}
# [doc = " A COMDAT section group in a [`CoffBigFile`](super::CoffBigFile)."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectComdat`] trait implementation."] pub type CoffBigComdat < 'data , 'file , R = & 'data [u8] > = CoffComdat < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
};
}
