// Generated macro for impl_91 (impl)
macro_rules! Depcrate_abi_exampleimpl_91 {
() => {
// Module: crate::abi_example
// Provides: {"impl_91"}
// Dependencies: {}
impl < T : AbiExample > AbiExample for std :: sync :: RwLock < T > { fn example () -> Self { println ! ("AbiExample for (RwLock<T>): {}" , type_name ::< Self > ()) ; std :: sync :: RwLock :: new (T :: example ()) } }
};
}
