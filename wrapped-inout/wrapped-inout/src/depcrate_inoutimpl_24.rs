// Generated macro for impl_24 (impl)
macro_rules! Depcrate_inoutimpl_24 {
() => {
// Module: crate::inout
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'inp , 'out , T > From < (& 'inp T , & 'out mut T) > for InOut < 'inp , 'out , T > { # [inline (always)] fn from ((in_val , out_val) : (& 'inp T , & 'out mut T)) -> Self { Self { in_ptr : in_val as * const T , out_ptr : out_val as * mut T , _pd : Default :: default () , } } }
};
}
