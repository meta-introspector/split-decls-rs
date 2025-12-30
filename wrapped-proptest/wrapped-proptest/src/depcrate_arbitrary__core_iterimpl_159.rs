// Generated macro for impl_159 (impl)
macro_rules! Depcrate_arbitrary__core_iterimpl_159 {
() => {
// Module: crate::arbitrary::_core::iter
// Provides: {"impl_159"}
// Dependencies: {}
impl < A : fmt :: Debug + Iterator , B : fmt :: Debug + Iterator > functor :: ArbitraryF2 < A , B > for Zip < A , B > { type Parameters = () ; fn lift2_with < AS , BS > (fst : AS , snd : BS , _args : Self :: Parameters ,) -> BoxedStrategy < Self > where AS : Strategy < Value = A > + 'static , BS : Strategy < Value = B > + 'static , { (fst , snd) . prop_map (| (a , b) | a . zip (b)) . boxed () } }
};
}
