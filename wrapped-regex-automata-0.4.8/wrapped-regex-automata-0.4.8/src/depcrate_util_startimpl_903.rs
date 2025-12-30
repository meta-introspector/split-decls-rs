// Generated macro for impl_903 (impl)
macro_rules! Depcrate_util_startimpl_903 {
() => {
// Module: crate::util::start
// Provides: {"impl_903"}
// Dependencies: {}
impl core :: fmt :: Debug for StartByteMap { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use crate :: util :: escape :: DebugByte ; write ! (f , "StartByteMap{{") ? ; for byte in 0 ..= 255 { if byte > 0 { write ! (f , ", ") ? ; } let start = self . map [usize :: from (byte)] ; write ! (f , "{:?} => {:?}" , DebugByte (byte) , start) ? ; } write ! (f , "}}") ? ; Ok (()) } }
};
}
