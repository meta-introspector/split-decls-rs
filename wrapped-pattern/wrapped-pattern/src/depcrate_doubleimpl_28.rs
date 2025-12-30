// Generated macro for impl_28 (impl)
macro_rules! Depcrate_doubleimpl_28 {
() => {
// Module: crate::double
// Provides: {"impl_28"}
// Dependencies: {}
impl < W0 , W1 > PlaceholderValueProvider < DoublePlaceholderKey > for (W0 , W1) where W0 : Writeable , W1 : Writeable , { type Error = Infallible ; type W < 'a > = WriteableAsTryWriteableInfallible < Either < & 'a W0 , & 'a W1 > > where Self : 'a ; type L < 'a , 'l > = & 'l str where Self : 'a ; # [inline] fn value_for (& self , key : DoublePlaceholderKey) -> Self :: W < '_ > { let writeable = match key { DoublePlaceholderKey :: Place0 => Either :: Left (& self . 0) , DoublePlaceholderKey :: Place1 => Either :: Right (& self . 1) , } ; WriteableAsTryWriteableInfallible (writeable) } # [inline] fn map_literal < 'a , 'l > (& 'a self , literal : & 'l str) -> Self :: L < 'a , 'l > { literal } }
};
}
