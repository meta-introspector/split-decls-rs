// Generated macro for impl_1058 (impl)
macro_rules! Depcrate_ir_trapcodeimpl_1058 {
() => {
// Module: crate::ir::trapcode
// Provides: {"impl_1058"}
// Dependencies: {}
impl FromStr for TrapCode { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "stk_ovf" => Ok (Self :: STACK_OVERFLOW) , "heap_oob" => Ok (Self :: HEAP_OUT_OF_BOUNDS) , "int_ovf" => Ok (Self :: INTEGER_OVERFLOW) , "int_divz" => Ok (Self :: INTEGER_DIVISION_BY_ZERO) , "bad_toint" => Ok (Self :: BAD_CONVERSION_TO_INTEGER) , _ if s . starts_with ("user") => { let num = s [4 ..] . parse () . map_err (| _ | ()) ? ; TrapCode :: user (num) . ok_or (()) } _ => Err (()) , } } }
};
}
