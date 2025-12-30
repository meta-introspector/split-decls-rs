// Generated macro for impl_356 (impl)
macro_rules! Depcrate_arbitrary__alloc_opsimpl_356 {
() => {
// Module: crate::arbitrary::_alloc::ops
// Provides: {"impl_356"}
// Dependencies: {}
# [cfg (feature = "unstable")] impl < A : fmt :: Debug + 'static , B : fmt :: Debug + 'static > functor :: ArbitraryF2 < A , B > for CoroutineState < A , B > { type Parameters = () ; fn lift2_with < AS , BS > (fst : AS , snd : BS , _args : Self :: Parameters ,) -> BoxedStrategy < Self > where AS : Strategy < Value = A > + 'static , BS : Strategy < Value = B > + 'static , { prop_oneof ! [fst . prop_map (CoroutineState :: Yielded) , snd . prop_map (CoroutineState :: Complete)] . boxed () } }
};
}
