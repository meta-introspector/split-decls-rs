// Generated macro for impl_156 (impl)
macro_rules! Depcrate_singleimpl_156 {
() => {
// Module: crate::single
// Provides: {"impl_156"}
// Dependencies: {}
impl < W > PlaceholderValueProvider < SinglePlaceholderKey > for (W ,) where W : Writeable , { type Error = Infallible ; type W < 'a > = WriteableAsTryWriteableInfallible < & 'a W > where Self : 'a ; type L < 'a , 'l > = & 'l str where Self : 'a ; fn value_for (& self , _key : SinglePlaceholderKey) -> Self :: W < '_ > { WriteableAsTryWriteableInfallible (& self . 0) } # [inline] fn map_literal < 'a , 'l > (& 'a self , literal : & 'l str) -> Self :: L < 'a , 'l > { literal } }
};
}
