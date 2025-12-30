// Generated macro for atomic_example_impls (macro)
macro_rules! Depcrate_abi_exampleatomic_example_impls {
() => {
// Module: crate::abi_example
// Provides: {"atomic_example_impls"}
// Dependencies: {}
macro_rules ! atomic_example_impls { ($ atomic_type : ident) => { impl AbiExample for $ atomic_type { fn example () -> Self { Self :: new (AbiExample :: example ()) } } } ; }
};
}
