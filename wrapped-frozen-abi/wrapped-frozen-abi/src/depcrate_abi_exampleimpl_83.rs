// Generated macro for impl_83 (impl)
macro_rules! Depcrate_abi_exampleimpl_83 {
() => {
// Module: crate::abi_example
// Provides: {"impl_83"}
// Dependencies: {}
impl < T : AbiExample > AbiExample for std :: sync :: Arc < T > { fn example () -> Self { println ! ("AbiExample for (Arc<T>): {}" , type_name ::< Self > ()) ; std :: sync :: Arc :: new (T :: example ()) } }
};
}
