// Generated macro for impl_640 (impl)
macro_rules! Depcrate_read_opimpl_640 {
() => {
// Module: crate::read::op
// Provides: {"impl_640"}
// Dependencies: {}
impl < R , Offset > Location < R , Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { # [doc = " Return true if the piece is empty."] pub fn is_empty (& self) -> bool { matches ! (* self , Location :: Empty) } }
};
}
