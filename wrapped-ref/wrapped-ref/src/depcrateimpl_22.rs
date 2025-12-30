// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'sval , V : sval :: Value + ? Sized > ValueRef < 'sval > for Ref < & 'sval V > { fn stream_ref < S : Stream < 'sval > + ? Sized > (& self , stream : & mut S) -> Result { self . 0 . stream (stream) } }
};
}
