// Generated macro for impl_27 (impl)
macro_rules! Depcrate_abi_exampleimpl_27 {
() => {
// Module: crate::abi_example
// Provides: {"impl_27"}
// Dependencies: {}
impl < const N : usize , T : AbiExample > AbiExample for [T ; N] { fn example () -> Self { std :: array :: from_fn (| _ | T :: example ()) } }
};
}
