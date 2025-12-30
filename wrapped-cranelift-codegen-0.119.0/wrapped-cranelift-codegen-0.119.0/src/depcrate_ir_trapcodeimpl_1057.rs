// Generated macro for impl_1057 (impl)
macro_rules! Depcrate_ir_trapcodeimpl_1057 {
() => {
// Module: crate::ir::trapcode
// Provides: {"impl_1057"}
// Dependencies: {}
impl Display for TrapCode { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { let identifier = match * self { Self :: STACK_OVERFLOW => "stk_ovf" , Self :: HEAP_OUT_OF_BOUNDS => "heap_oob" , Self :: INTEGER_OVERFLOW => "int_ovf" , Self :: INTEGER_DIVISION_BY_ZERO => "int_divz" , Self :: BAD_CONVERSION_TO_INTEGER => "bad_toint" , TrapCode (x) => return write ! (f , "user{x}") , } ; f . write_str (identifier) } }
};
}
