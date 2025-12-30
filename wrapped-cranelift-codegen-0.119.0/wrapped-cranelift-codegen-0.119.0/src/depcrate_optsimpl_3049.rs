// Generated macro for impl_3049 (impl)
macro_rules! Depcrate_optsimpl_3049 {
() => {
// Module: crate::opts
// Provides: {"impl_3049"}
// Dependencies: {}
impl < 'a , 'b , 'c > ContextIter for MaybeUnaryEtorIter < 'a , 'b , 'c > where 'b : 'a , 'c : 'b , { type Context = IsleContext < 'a , 'b , 'c > ; type Output = (Type , Value) ; fn next (& mut self , ctx : & mut IsleContext < 'a , 'b , 'c >) -> Option < Self :: Output > { debug_assert_ne ! (self . opcode , None) ; while let Some ((ty , inst_def)) = self . inner . next (ctx) { let InstructionData :: Unary { opcode , arg } = inst_def else { continue ; } ; if Some (opcode) == self . opcode { self . fallback = None ; return Some ((ty , arg)) ; } } self . fallback . take () . map (| value | { let ty = generated_code :: Context :: value_type (ctx , value) ; (ty , value) }) } }
};
}
