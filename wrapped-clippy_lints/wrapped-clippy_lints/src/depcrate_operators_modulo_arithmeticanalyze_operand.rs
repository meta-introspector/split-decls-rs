// Generated macro for analyze_operand (function)
macro_rules! Depcrate_operators_modulo_arithmeticanalyze_operand {
() => {
// Module: crate::operators::modulo_arithmetic
// Provides: {"analyze_operand"}
// Dependencies: {}
fn analyze_operand (operand : & Expr < '_ > , cx : & LateContext < '_ > , expr : & Expr < '_ >) -> Option < OperandInfo > { match ConstEvalCtxt :: new (cx) . eval (operand) ? { Constant :: Int (v) => match * cx . typeck_results () . expr_ty (expr) . kind () { ty :: Int (ity) => { let value : i128 = sext (cx . tcx , v , ity) ; Some (OperandInfo { string_representation : Some (value . to_string ()) , is_negative : value < 0 , is_integral : true , }) } , ty :: Uint (_) => Some (OperandInfo { string_representation : None , is_negative : false , is_integral : true , }) , _ => None , } , Constant :: F32 (f) => Some (floating_point_operand_info (& f)) , Constant :: F64 (f) => Some (floating_point_operand_info (& f)) , _ => None , } }
};
}
