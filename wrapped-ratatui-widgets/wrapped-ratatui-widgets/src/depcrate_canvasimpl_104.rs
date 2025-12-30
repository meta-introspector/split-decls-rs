// Generated macro for impl_104 (impl)
macro_rules! Depcrate_canvasimpl_104 {
() => {
// Module: crate::canvas
// Provides: {"impl_104"}
// Dependencies: {}
impl < 'a , 'b > From < & 'a mut Context < 'b > > for Painter < 'a , 'b > { fn from (context : & 'a mut Context < 'b >) -> Self { let resolution = context . grid . resolution () ; Self { context , resolution , } } }
};
}
