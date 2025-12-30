// Generated macro for format_f16_return_value (function)
macro_rules! Depcrate_common_intrinsicformat_f16_return_value {
() => {
// Module: crate::common::intrinsic
// Provides: {"format_f16_return_value"}
// Dependencies: {}
pub fn format_f16_return_value < T : IntrinsicTypeDefinition > (intrinsic : & dyn IntrinsicDefinition < T > ,) -> String { let return_value = match intrinsic . results () . kind () { TypeKind :: Float if intrinsic . results () . inner_size () == 16 => "debug_f16(__return_value)" , _ => "format_args!(\"{__return_value:.150?}\")" , } ; String :: from (return_value) }
};
}
