// Generated macro for impl_81 (impl)
macro_rules! Depcrate_abi_exampleimpl_81 {
() => {
// Module: crate::abi_example
// Provides: {"impl_81"}
// Dependencies: {}
impl < T : AbiExample > AbiExample for Box < [T] > { fn example () -> Self { println ! ("AbiExample for (Box<[T]>): {}" , type_name ::< Self > ()) ; Box :: new ([T :: example ()]) } }
};
}
