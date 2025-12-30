// Generated macro for impl_162 (impl)
macro_rules! Depcrate_arbitrary__core_iterimpl_162 {
() => {
// Module: crate::arbitrary::_core::iter
// Provides: {"impl_162"}
// Dependencies: {}
impl < T , A : fmt :: Debug + Iterator < Item = T > , B : fmt :: Debug + Iterator < Item = T > , > functor :: ArbitraryF2 < A , B > for Chain < A , B > { type Parameters = () ; fn lift2_with < AS , BS > (fst : AS , snd : BS , _args : Self :: Parameters ,) -> BoxedStrategy < Self > where AS : Strategy < Value = A > + 'static , BS : Strategy < Value = B > + 'static , { (fst , snd) . prop_map (| (a , b) | a . chain (b)) . boxed () } }
};
}
