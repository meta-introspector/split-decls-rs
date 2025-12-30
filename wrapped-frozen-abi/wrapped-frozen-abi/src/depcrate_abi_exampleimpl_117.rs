// Generated macro for impl_117 (impl)
macro_rules! Depcrate_abi_exampleimpl_117 {
() => {
// Module: crate::abi_example
// Provides: {"impl_117"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl < T : std :: cmp :: Eq + std :: hash :: Hash + AbiExample , S : AbiExample , H : std :: hash :: BuildHasher + Default + std :: clone :: Clone , > AbiExample for dashmap :: DashMap < T , S , H > { fn example () -> Self { println ! ("AbiExample for (DashMap<T, S, H>): {}" , type_name ::< Self > ()) ; let map = dashmap :: DashMap :: default () ; map . insert (T :: example () , S :: example ()) ; map } }
};
}
