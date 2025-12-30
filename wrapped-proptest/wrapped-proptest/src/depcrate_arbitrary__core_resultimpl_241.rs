// Generated macro for impl_241 (impl)
macro_rules! Depcrate_arbitrary__core_resultimpl_241 {
() => {
// Module: crate::arbitrary::_core::result
// Provides: {"impl_241"}
// Dependencies: {}
impl < A : fmt :: Debug , B : fmt :: Debug > functor :: ArbitraryF2 < A , B > for Result < A , B > { type Parameters = Probability ; fn lift2_with < AS , BS > (fst : AS , snd : BS , args : Self :: Parameters ,) -> BoxedStrategy < Self > where AS : Strategy < Value = A > + 'static , BS : Strategy < Value = B > + 'static , { maybe_ok_weighted (args , fst , snd) . boxed () } }
};
}
