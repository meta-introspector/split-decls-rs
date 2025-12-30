// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl From < LamportsError > for InstructionError { fn from (error : LamportsError) -> Self { match error { LamportsError :: ArithmeticOverflow => InstructionError :: ArithmeticOverflow , LamportsError :: ArithmeticUnderflow => InstructionError :: ArithmeticOverflow , } } }
};
}
