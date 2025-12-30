// Generated macro for impl_76 (impl)
macro_rules! Depcrate_abi_exampleimpl_76 {
() => {
// Module: crate::abi_example
// Provides: {"impl_76"}
// Dependencies: {}
impl < T : AbiExample > AbiExample for Option < T > { fn example () -> Self { println ! ("AbiExample for (Option<T>): {}" , type_name ::< Self > ()) ; Some (T :: example ()) } }
};
}
