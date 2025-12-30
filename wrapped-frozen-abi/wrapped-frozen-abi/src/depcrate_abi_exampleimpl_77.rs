// Generated macro for impl_77 (impl)
macro_rules! Depcrate_abi_exampleimpl_77 {
() => {
// Module: crate::abi_example
// Provides: {"impl_77"}
// Dependencies: {}
impl < O : AbiExample , E : AbiExample > AbiExample for Result < O , E > { fn example () -> Self { println ! ("AbiExample for (Result<O, E>): {}" , type_name ::< Self > ()) ; Ok (O :: example ()) } }
};
}
