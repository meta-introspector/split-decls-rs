// Generated macro for impl_95 (impl)
macro_rules! Depcrate_abi_exampleimpl_95 {
() => {
// Module: crate::abi_example
// Provides: {"impl_95"}
// Dependencies: {}
impl < T : std :: cmp :: Ord + AbiExample , S : AbiExample > AbiExample for BTreeMap < T , S > { fn example () -> Self { println ! ("AbiExample for (BTreeMap<T, S>): {}" , type_name ::< Self > ()) ; let mut map = BTreeMap :: default () ; map . insert (T :: example () , S :: example ()) ; map } }
};
}
