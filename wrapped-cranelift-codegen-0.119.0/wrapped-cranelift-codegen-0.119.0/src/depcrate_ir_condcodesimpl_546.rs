// Generated macro for impl_546 (impl)
macro_rules! Depcrate_ir_condcodesimpl_546 {
() => {
// Module: crate::ir::condcodes
// Provides: {"impl_546"}
// Dependencies: {}
impl Display for FloatCC { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { use self :: FloatCC :: * ; f . write_str (match * self { Ordered => "ord" , Unordered => "uno" , Equal => "eq" , NotEqual => "ne" , OrderedNotEqual => "one" , UnorderedOrEqual => "ueq" , LessThan => "lt" , LessThanOrEqual => "le" , GreaterThan => "gt" , GreaterThanOrEqual => "ge" , UnorderedOrLessThan => "ult" , UnorderedOrLessThanOrEqual => "ule" , UnorderedOrGreaterThan => "ugt" , UnorderedOrGreaterThanOrEqual => "uge" , }) } }
};
}
