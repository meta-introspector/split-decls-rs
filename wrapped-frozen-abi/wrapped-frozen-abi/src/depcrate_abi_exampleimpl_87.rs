// Generated macro for impl_87 (impl)
macro_rules! Depcrate_abi_exampleimpl_87 {
() => {
// Module: crate::abi_example
// Provides: {"impl_87"}
// Dependencies: {}
impl < T : AbiExample > AbiExample for std :: sync :: Weak < T > { fn example () -> Self { println ! ("AbiExample for (Arc's Weak<T>): {}" , type_name ::< Self > ()) ; std :: sync :: Arc :: downgrade (leak_and_inhibit_drop (std :: sync :: Arc :: new (T :: example ()))) } }
};
}
