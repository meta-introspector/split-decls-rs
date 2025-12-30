// Generated macro for impl_490 (impl)
macro_rules! Depcrate_fmt_temporal_piecesimpl_490 {
() => {
// Module: crate::fmt::temporal::pieces
// Provides: {"impl_490"}
// Dependencies: {}
impl < 'n > core :: fmt :: Display for Pieces < 'n > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use crate :: fmt :: StdFmtWrite ; let precision = f . precision () . map (| p | u8 :: try_from (p) . unwrap_or (u8 :: MAX)) ; super :: DateTimePrinter :: new () . precision (precision) . print_pieces (self , StdFmtWrite (f)) . map_err (| _ | core :: fmt :: Error) } }
};
}
