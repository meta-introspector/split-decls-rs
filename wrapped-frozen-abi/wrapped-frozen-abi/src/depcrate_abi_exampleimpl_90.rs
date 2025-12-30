// Generated macro for impl_90 (impl)
macro_rules! Depcrate_abi_exampleimpl_90 {
() => {
// Module: crate::abi_example
// Provides: {"impl_90"}
// Dependencies: {}
impl < T : AbiExample > AbiExample for std :: sync :: Mutex < T > { fn example () -> Self { println ! ("AbiExample for (Mutex<T>): {}" , type_name ::< Self > ()) ; std :: sync :: Mutex :: new (T :: example ()) } }
};
}
