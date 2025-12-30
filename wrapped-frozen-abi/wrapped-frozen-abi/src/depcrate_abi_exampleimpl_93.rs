// Generated macro for impl_93 (impl)
macro_rules! Depcrate_abi_exampleimpl_93 {
() => {
// Module: crate::abi_example
// Provides: {"impl_93"}
// Dependencies: {}
impl < T : std :: cmp :: Eq + std :: hash :: Hash + AbiExample , S : AbiExample , H : std :: hash :: BuildHasher + Default , > AbiExample for HashMap < T , S , H > { fn example () -> Self { println ! ("AbiExample for (HashMap<T, S, H>): {}" , type_name ::< Self > ()) ; let mut map = HashMap :: default () ; map . insert (T :: example () , S :: example ()) ; map } }
};
}
