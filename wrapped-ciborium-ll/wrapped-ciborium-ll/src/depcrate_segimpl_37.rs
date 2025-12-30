// Generated macro for impl_37 (impl)
macro_rules! Depcrate_segimpl_37 {
() => {
// Module: crate::seg
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'r , R , P > Segments < 'r , R , P > { # [inline] pub (crate) fn new (decoder : & 'r mut Decoder < R > , unwrap : fn (Header) -> Result < Option < usize > , () > ,) -> Self { Self { reader : decoder , state : State :: Initial , parser : PhantomData , unwrap , } } }
};
}
