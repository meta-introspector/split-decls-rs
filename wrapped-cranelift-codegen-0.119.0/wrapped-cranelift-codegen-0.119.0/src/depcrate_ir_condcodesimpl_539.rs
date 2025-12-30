// Generated macro for impl_539 (impl)
macro_rules! Depcrate_ir_condcodesimpl_539 {
() => {
// Module: crate::ir::condcodes
// Provides: {"impl_539"}
// Dependencies: {}
impl CondCode for IntCC { fn complement (self) -> Self { use self :: IntCC :: * ; match self { Equal => NotEqual , NotEqual => Equal , SignedLessThan => SignedGreaterThanOrEqual , SignedGreaterThanOrEqual => SignedLessThan , SignedGreaterThan => SignedLessThanOrEqual , SignedLessThanOrEqual => SignedGreaterThan , UnsignedLessThan => UnsignedGreaterThanOrEqual , UnsignedGreaterThanOrEqual => UnsignedLessThan , UnsignedGreaterThan => UnsignedLessThanOrEqual , UnsignedLessThanOrEqual => UnsignedGreaterThan , } } fn swap_args (self) -> Self { use self :: IntCC :: * ; match self { Equal => Equal , NotEqual => NotEqual , SignedGreaterThan => SignedLessThan , SignedGreaterThanOrEqual => SignedLessThanOrEqual , SignedLessThan => SignedGreaterThan , SignedLessThanOrEqual => SignedGreaterThanOrEqual , UnsignedGreaterThan => UnsignedLessThan , UnsignedGreaterThanOrEqual => UnsignedLessThanOrEqual , UnsignedLessThan => UnsignedGreaterThan , UnsignedLessThanOrEqual => UnsignedGreaterThanOrEqual , } } }
};
}
