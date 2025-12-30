// Generated macro for impl_547 (impl)
macro_rules! Depcrate_ir_condcodesimpl_547 {
() => {
// Module: crate::ir::condcodes
// Provides: {"impl_547"}
// Dependencies: {}
impl FromStr for FloatCC { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { use self :: FloatCC :: * ; match s { "ord" => Ok (Ordered) , "uno" => Ok (Unordered) , "eq" => Ok (Equal) , "ne" => Ok (NotEqual) , "one" => Ok (OrderedNotEqual) , "ueq" => Ok (UnorderedOrEqual) , "lt" => Ok (LessThan) , "le" => Ok (LessThanOrEqual) , "gt" => Ok (GreaterThan) , "ge" => Ok (GreaterThanOrEqual) , "ult" => Ok (UnorderedOrLessThan) , "ule" => Ok (UnorderedOrLessThanOrEqual) , "ugt" => Ok (UnorderedOrGreaterThan) , "uge" => Ok (UnorderedOrGreaterThanOrEqual) , _ => Err (()) , } } }
};
}
