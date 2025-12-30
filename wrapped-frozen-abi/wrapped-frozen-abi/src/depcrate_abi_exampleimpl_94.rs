// Generated macro for impl_94 (impl)
macro_rules! Depcrate_abi_exampleimpl_94 {
() => {
// Module: crate::abi_example
// Provides: {"impl_94"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl < T : Clone + std :: cmp :: Eq + std :: hash :: Hash + AbiExample , S : Clone + AbiExample , H : std :: hash :: BuildHasher + Default , > AbiExample for im :: HashMap < T , S , H > { fn example () -> Self { println ! ("AbiExample for (HashMap<T, S, H>): {}" , type_name ::< Self > ()) ; let mut map = im :: HashMap :: default () ; map . insert (T :: example () , S :: example ()) ; map } }
};
}
