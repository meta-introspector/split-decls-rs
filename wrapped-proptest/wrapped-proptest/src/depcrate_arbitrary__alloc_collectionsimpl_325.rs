// Generated macro for impl_325 (impl)
macro_rules! Depcrate_arbitrary__alloc_collectionsimpl_325 {
() => {
// Module: crate::arbitrary::_alloc::collections
// Provides: {"impl_325"}
// Dependencies: {}
# [cfg (feature = "std")] impl < A : fmt :: Debug + Eq + Hash + 'static , B : fmt :: Debug + 'static > functor :: ArbitraryF2 < A , B > for hash_map :: IntoIter < A , B > { type Parameters = SizeRange ; fn lift2_with < AS , BS > (fst : AS , snd : BS , args : Self :: Parameters ,) -> BoxedStrategy < Self > where AS : Strategy < Value = A > + 'static , BS : Strategy < Value = B > + 'static , { static_map (hash_map (fst , snd , args) , HashMap :: into_iter) . boxed () } }
};
}
