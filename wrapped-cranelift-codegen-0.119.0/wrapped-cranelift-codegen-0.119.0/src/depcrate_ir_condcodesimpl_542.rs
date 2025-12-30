// Generated macro for impl_542 (impl)
macro_rules! Depcrate_ir_condcodesimpl_542 {
() => {
// Module: crate::ir::condcodes
// Provides: {"impl_542"}
// Dependencies: {}
impl FromStr for IntCC { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { use self :: IntCC :: * ; match s { "eq" => Ok (Equal) , "ne" => Ok (NotEqual) , "sge" => Ok (SignedGreaterThanOrEqual) , "sgt" => Ok (SignedGreaterThan) , "sle" => Ok (SignedLessThanOrEqual) , "slt" => Ok (SignedLessThan) , "uge" => Ok (UnsignedGreaterThanOrEqual) , "ugt" => Ok (UnsignedGreaterThan) , "ule" => Ok (UnsignedLessThanOrEqual) , "ult" => Ok (UnsignedLessThan) , _ => Err (()) , } } }
};
}
