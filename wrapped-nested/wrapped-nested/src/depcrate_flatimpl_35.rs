// Generated macro for impl_35 (impl)
macro_rules! Depcrate_flatimpl_35 {
() => {
// Module: crate::flat
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'sval , S : Stream < 'sval > > fmt :: Debug for EnumVariant < 'sval , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { EnumVariant :: Tagged (stream) => fmt :: Debug :: fmt (stream , f) , EnumVariant :: Tuple (stream) => fmt :: Debug :: fmt (stream , f) , EnumVariant :: Record (stream) => fmt :: Debug :: fmt (stream , f) , } } }
};
}
