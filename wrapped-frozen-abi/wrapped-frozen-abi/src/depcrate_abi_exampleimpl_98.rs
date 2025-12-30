// Generated macro for impl_98 (impl)
macro_rules! Depcrate_abi_exampleimpl_98 {
() => {
// Module: crate::abi_example
// Provides: {"impl_98"}
// Dependencies: {}
impl < T : std :: cmp :: Eq + std :: hash :: Hash + AbiExample , H : std :: hash :: BuildHasher + Default > AbiExample for HashSet < T , H > { fn example () -> Self { println ! ("AbiExample for (HashSet<T, H>): {}" , type_name ::< Self > ()) ; let mut set : HashSet < T , H > = HashSet :: default () ; set . insert (T :: example ()) ; set } }
};
}
