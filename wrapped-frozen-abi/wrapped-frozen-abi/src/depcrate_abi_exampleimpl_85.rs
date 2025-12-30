// Generated macro for impl_85 (impl)
macro_rules! Depcrate_abi_exampleimpl_85 {
() => {
// Module: crate::abi_example
// Provides: {"impl_85"}
// Dependencies: {}
impl < T : AbiExample > AbiExample for & T { fn example () -> Self { println ! ("AbiExample for (&T): {}" , type_name ::< Self > ()) ; leak_and_inhibit_drop (T :: example ()) } }
};
}
