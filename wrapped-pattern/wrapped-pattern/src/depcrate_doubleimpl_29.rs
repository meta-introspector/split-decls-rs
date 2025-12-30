// Generated macro for impl_29 (impl)
macro_rules! Depcrate_doubleimpl_29 {
() => {
// Module: crate::double
// Provides: {"impl_29"}
// Dependencies: {}
impl < W > PlaceholderValueProvider < DoublePlaceholderKey > for [W ; 2] where W : Writeable , { type Error = Infallible ; type W < 'a > = WriteableAsTryWriteableInfallible < & 'a W > where Self : 'a ; type L < 'a , 'l > = & 'l str where Self : 'a ; # [inline] fn value_for (& self , key : DoublePlaceholderKey) -> Self :: W < '_ > { let [item0 , item1] = self ; let writeable = match key { DoublePlaceholderKey :: Place0 => item0 , DoublePlaceholderKey :: Place1 => item1 , } ; WriteableAsTryWriteableInfallible (writeable) } # [inline] fn map_literal < 'a , 'l > (& 'a self , literal : & 'l str) -> Self :: L < 'a , 'l > { literal } }
};
}
