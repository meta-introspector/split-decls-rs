// Generated macro for impl_3048 (impl)
macro_rules! Depcrate_optsimpl_3048 {
() => {
// Module: crate::opts
// Provides: {"impl_3048"}
// Dependencies: {}
impl MaybeUnaryEtorIter < '_ , '_ , '_ > { fn new (opcode : Opcode , value : Value) -> Self { debug_assert_eq ! (opcode . format () , InstructionFormat :: Unary) ; Self { opcode : Some (opcode) , inner : InstDataEtorIter :: new (value) , fallback : Some (value) , } } }
};
}
