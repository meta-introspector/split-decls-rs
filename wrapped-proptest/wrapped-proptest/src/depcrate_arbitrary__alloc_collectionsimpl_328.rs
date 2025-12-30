// Generated macro for impl_328 (impl)
macro_rules! Depcrate_arbitrary__alloc_collectionsimpl_328 {
() => {
// Module: crate::arbitrary::_alloc::collections
// Provides: {"impl_328"}
// Dependencies: {}
impl < A : fmt :: Debug + Ord , B : fmt :: Debug > functor :: ArbitraryF2 < A , B > for BTreeMap < A , B > { type Parameters = SizeRange ; fn lift2_with < AS , BS > (fst : AS , snd : BS , args : Self :: Parameters ,) -> BoxedStrategy < Self > where AS : Strategy < Value = A > + 'static , BS : Strategy < Value = B > + 'static , { btree_map (fst , snd , args) . boxed () } }
};
}
