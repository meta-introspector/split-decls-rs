// Generated macro for impl_330 (impl)
macro_rules! Depcrate_arbitrary__alloc_collectionsimpl_330 {
() => {
// Module: crate::arbitrary::_alloc::collections
// Provides: {"impl_330"}
// Dependencies: {}
impl < A : fmt :: Debug + Ord + 'static , B : fmt :: Debug + 'static > functor :: ArbitraryF2 < A , B > for btree_map :: IntoIter < A , B > { type Parameters = SizeRange ; fn lift2_with < AS , BS > (fst : AS , snd : BS , args : Self :: Parameters ,) -> BoxedStrategy < Self > where AS : Strategy < Value = A > + 'static , BS : Strategy < Value = B > + 'static , { static_map (btree_map (fst , snd , args) , BTreeMap :: into_iter) . boxed () } }
};
}
