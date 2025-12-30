// Generated macro for impl_575 (impl)
macro_rules! Depcrate_interpret_operandimpl_575 {
() => {
// Module: crate::interpret::operand
// Provides: {"impl_575"}
// Dependencies: {}
impl < Prov : Provenance > std :: fmt :: Display for ImmTy < '_ , Prov > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { # [doc = " Helper function for printing a scalar to a FmtPrinter"] fn print_scalar < 'a , 'tcx , Prov : Provenance > (p : & mut FmtPrinter < 'a , 'tcx > , s : Scalar < Prov > , ty : Ty < 'tcx > ,) -> Result < () , std :: fmt :: Error > { match s { Scalar :: Int (int) => p . pretty_print_const_scalar_int (int , ty , true) , Scalar :: Ptr (ptr , _sz) => { p . pretty_print_const_pointer (ptr , ty) } } } ty :: tls :: with (| tcx | { match self . imm { Immediate :: Scalar (s) => { if let Some (ty) = tcx . lift (self . layout . ty) { let s = FmtPrinter :: print_string (tcx , Namespace :: ValueNS , | p | { print_scalar (p , s , ty) }) ? ; f . write_str (& s) ? ; return Ok (()) ; } write ! (f , "{:x}: {}" , s , self . layout . ty) } Immediate :: ScalarPair (a , b) => { write ! (f , "({:x}, {:x}): {}" , a , b , self . layout . ty) } Immediate :: Uninit => { write ! (f , "uninit: {}" , self . layout . ty) } } }) } }
};
}
