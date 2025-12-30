// Generated macro for impl_15 (impl)
macro_rules! Depcrate_serialimpl_15 {
() => {
// Module: crate::serial
// Provides: {"impl_15"}
// Dependencies: {}
# [doc = " Implementation of `core::fmt::Write` for the HAL's `serial::Write`."] # [doc = ""] # [doc = " TODO write example of usage"] impl < Word , Error : self :: Error > core :: fmt :: Write for dyn Write < Word , Error = Error > + '_ where Word : Copy + From < u8 > , { # [inline] fn write_str (& mut self , s : & str) -> core :: fmt :: Result { for c in s . bytes () { nb :: block ! (self . write (Word :: from (c))) . map_err (| _ | core :: fmt :: Error) ? ; } Ok (()) } }
};
}
