// Generated macro for impl_1738 (impl)
macro_rules! Depcrate_isa_aarch64_inst_argsimpl_1738 {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"impl_1738"}
// Dependencies: {}
impl CondBrKind { # [doc = " Return the inverted branch condition."] pub fn invert (self) -> CondBrKind { match self { CondBrKind :: Zero (reg , size) => CondBrKind :: NotZero (reg , size) , CondBrKind :: NotZero (reg , size) => CondBrKind :: Zero (reg , size) , CondBrKind :: Cond (c) => CondBrKind :: Cond (c . invert ()) , } } }
};
}
