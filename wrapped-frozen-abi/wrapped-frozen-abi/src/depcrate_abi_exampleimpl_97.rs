// Generated macro for impl_97 (impl)
macro_rules! Depcrate_abi_exampleimpl_97 {
() => {
// Module: crate::abi_example
// Provides: {"impl_97"}
// Dependencies: {}
impl < T : AbiExample > AbiExample for VecDeque < T > { fn example () -> Self { println ! ("AbiExample for (Vec<T>): {}" , type_name ::< Self > ()) ; VecDeque :: from (vec ! [T :: example ()]) } }
};
}
