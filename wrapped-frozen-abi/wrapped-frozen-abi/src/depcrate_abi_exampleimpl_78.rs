// Generated macro for impl_78 (impl)
macro_rules! Depcrate_abi_exampleimpl_78 {
() => {
// Module: crate::abi_example
// Provides: {"impl_78"}
// Dependencies: {}
impl < T : AbiExample > AbiExample for Box < T > { fn example () -> Self { println ! ("AbiExample for (Box<T>): {}" , type_name ::< Self > ()) ; Box :: new (T :: example ()) } }
};
}
