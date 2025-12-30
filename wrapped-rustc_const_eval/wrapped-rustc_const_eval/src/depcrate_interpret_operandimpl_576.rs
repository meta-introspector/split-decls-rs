// Generated macro for impl_576 (impl)
macro_rules! Depcrate_interpret_operandimpl_576 {
() => {
// Module: crate::interpret::operand
// Provides: {"impl_576"}
// Dependencies: {}
impl < Prov : Provenance > std :: fmt :: Debug for ImmTy < '_ , Prov > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("ImmTy") . field ("imm" , & self . imm) . field ("ty" , & format_args ! ("{}" , self . layout . ty)) . finish () } }
};
}
