// Generated macro for impl_96 (impl)
macro_rules! Depcrate_abi_exampleimpl_96 {
() => {
// Module: crate::abi_example
// Provides: {"impl_96"}
// Dependencies: {}
impl < T : AbiExample > AbiExample for Vec < T > { fn example () -> Self { println ! ("AbiExample for (Vec<T>): {}" , type_name ::< Self > ()) ; vec ! [T :: example ()] } }
};
}
