// Generated macro for impl_86 (impl)
macro_rules! Depcrate_abi_exampleimpl_86 {
() => {
// Module: crate::abi_example
// Provides: {"impl_86"}
// Dependencies: {}
impl < T : AbiExample > AbiExample for & [T] { fn example () -> Self { println ! ("AbiExample for (&[T]): {}" , type_name ::< Self > ()) ; leak_and_inhibit_drop (vec ! [T :: example ()]) } }
};
}
