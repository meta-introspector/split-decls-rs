// Generated macro for impl_582 (impl)
macro_rules! Depcrate_interpret_operandimpl_582 {
() => {
// Module: crate::interpret::operand
// Provides: {"impl_582"}
// Dependencies: {}
impl < Prov : Provenance > std :: fmt :: Debug for OpTy < '_ , Prov > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("OpTy") . field ("op" , & self . op) . field ("ty" , & format_args ! ("{}" , self . layout . ty)) . finish () } }
};
}
