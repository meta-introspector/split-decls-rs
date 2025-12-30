// Generated macro for floating_point_operand_info (function)
macro_rules! Depcrate_operators_modulo_arithmeticfloating_point_operand_info {
() => {
// Module: crate::operators::modulo_arithmetic
// Provides: {"floating_point_operand_info"}
// Dependencies: {}
fn floating_point_operand_info < T : Display + PartialOrd + From < f32 > > (f : & T) -> OperandInfo { OperandInfo { string_representation : Some (format ! ("{:.3}" , * f)) , is_negative : * f < 0.0 . into () , is_integral : false , } }
};
}
