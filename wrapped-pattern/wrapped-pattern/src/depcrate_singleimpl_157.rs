// Generated macro for impl_157 (impl)
macro_rules! Depcrate_singleimpl_157 {
() => {
// Module: crate::single
// Provides: {"impl_157"}
// Dependencies: {}
impl < W > PlaceholderValueProvider < SinglePlaceholderKey > for [W ; 1] where W : Writeable , { type Error = Infallible ; type W < 'a > = WriteableAsTryWriteableInfallible < & 'a W > where Self : 'a ; type L < 'a , 'l > = & 'l str where Self : 'a ; fn value_for (& self , _key : SinglePlaceholderKey) -> Self :: W < '_ > { let [value] = self ; WriteableAsTryWriteableInfallible (value) } # [inline] fn map_literal < 'a , 'l > (& 'a self , literal : & 'l str) -> Self :: L < 'a , 'l > { literal } }
};
}
