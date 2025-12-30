// Generated macro for impl_545 (impl)
macro_rules! Depcrate_ir_condcodesimpl_545 {
() => {
// Module: crate::ir::condcodes
// Provides: {"impl_545"}
// Dependencies: {}
impl CondCode for FloatCC { fn complement (self) -> Self { use self :: FloatCC :: * ; match self { Ordered => Unordered , Unordered => Ordered , Equal => NotEqual , NotEqual => Equal , OrderedNotEqual => UnorderedOrEqual , UnorderedOrEqual => OrderedNotEqual , LessThan => UnorderedOrGreaterThanOrEqual , LessThanOrEqual => UnorderedOrGreaterThan , GreaterThan => UnorderedOrLessThanOrEqual , GreaterThanOrEqual => UnorderedOrLessThan , UnorderedOrLessThan => GreaterThanOrEqual , UnorderedOrLessThanOrEqual => GreaterThan , UnorderedOrGreaterThan => LessThanOrEqual , UnorderedOrGreaterThanOrEqual => LessThan , } } fn swap_args (self) -> Self { use self :: FloatCC :: * ; match self { Ordered => Ordered , Unordered => Unordered , Equal => Equal , NotEqual => NotEqual , OrderedNotEqual => OrderedNotEqual , UnorderedOrEqual => UnorderedOrEqual , LessThan => GreaterThan , LessThanOrEqual => GreaterThanOrEqual , GreaterThan => LessThan , GreaterThanOrEqual => LessThanOrEqual , UnorderedOrLessThan => UnorderedOrGreaterThan , UnorderedOrLessThanOrEqual => UnorderedOrGreaterThanOrEqual , UnorderedOrGreaterThan => UnorderedOrLessThan , UnorderedOrGreaterThanOrEqual => UnorderedOrLessThanOrEqual , } } }
};
}
