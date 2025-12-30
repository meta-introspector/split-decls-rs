// Generated macro for impl_296 (impl)
macro_rules! Depcrate_enc_range_encimpl_296 {
() => {
// Module: crate::enc::range_enc
// Provides: {"impl_296"}
// Dependencies: {}
impl RangeEncoderBuffer { pub (crate) fn new (size : usize) -> Self { Self { buf : vec ! [0 ; size] , pos : 0 , } } pub (crate) fn write_to < W : Write > (& self , out : & mut W) -> crate :: Result < () > { out . write_all (& self . buf [.. self . pos]) } }
};
}
