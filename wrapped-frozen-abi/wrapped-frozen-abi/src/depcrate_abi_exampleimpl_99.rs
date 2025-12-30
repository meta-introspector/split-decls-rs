// Generated macro for impl_99 (impl)
macro_rules! Depcrate_abi_exampleimpl_99 {
() => {
// Module: crate::abi_example
// Provides: {"impl_99"}
// Dependencies: {}
impl < T : std :: cmp :: Ord + AbiExample > AbiExample for BTreeSet < T > { fn example () -> Self { println ! ("AbiExample for (BTreeSet<T>): {}" , type_name ::< Self > ()) ; let mut set : BTreeSet < T > = BTreeSet :: default () ; set . insert (T :: example ()) ; set } }
};
}
