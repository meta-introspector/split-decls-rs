// Generated macro for Writer (struct)
macro_rules! Depcrate_zioWriter {
() => {
// Module: crate::zio
// Provides: {"Writer"}
// Dependencies: {}
# [derive (Debug)] pub struct Writer < W : Write , D : Ops > { obj : Option < W > , pub data : D , buf : Vec < u8 > , }
};
}
