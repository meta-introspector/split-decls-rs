// Generated macro for impl_18 (impl)
macro_rules! Depcrate_flatimpl_18 {
() => {
// Module: crate::flat
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'sval , S : Stream < 'sval > > fmt :: Debug for State < 'sval , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { State :: Any (state) => fmt :: Debug :: fmt (state , f) , State :: Seq (state) => fmt :: Debug :: fmt (state , f) , State :: Map (state) => fmt :: Debug :: fmt (state , f) , State :: Tuple (state) => fmt :: Debug :: fmt (state , f) , State :: Record (state) => fmt :: Debug :: fmt (state , f) , State :: Tagged (state) => fmt :: Debug :: fmt (state , f) , State :: Enum (state) => fmt :: Debug :: fmt (state , f) , State :: EnumVariant (state) => fmt :: Debug :: fmt (state , f) , State :: Done (_) => f . debug_struct ("Done") . finish_non_exhaustive () , } } }
};
}
