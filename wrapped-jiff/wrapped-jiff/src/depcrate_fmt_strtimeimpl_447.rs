// Generated macro for impl_447 (impl)
macro_rules! Depcrate_fmt_strtimeimpl_447 {
() => {
// Module: crate::fmt::strtime
// Provides: {"impl_447"}
// Dependencies: {}
impl < 'f > core :: fmt :: Display for Display < 'f > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use crate :: fmt :: StdFmtWrite ; self . tm . format (self . fmt , StdFmtWrite (f)) . map_err (| _ | core :: fmt :: Error) } }
};
}
