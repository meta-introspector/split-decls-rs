// Generated macro for impl_324 (impl)
macro_rules! Depcrate_arbitrary__alloc_collectionsimpl_324 {
() => {
// Module: crate::arbitrary::_alloc::collections
// Provides: {"impl_324"}
// Dependencies: {}
# [cfg (feature = "std")] impl < A : fmt :: Debug + Eq + Hash , B : fmt :: Debug > functor :: ArbitraryF2 < A , B > for HashMap < A , B > { type Parameters = SizeRange ; fn lift2_with < AS , BS > (fst : AS , snd : BS , args : Self :: Parameters ,) -> BoxedStrategy < Self > where AS : Strategy < Value = A > + 'static , BS : Strategy < Value = B > + 'static , { hash_map (fst , snd , args) . boxed () } }
};
}
