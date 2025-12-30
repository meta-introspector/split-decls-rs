// Generated macro for array_span_lint (function)
macro_rules! Depcrate_zero_repeat_side_effectsarray_span_lint {
() => {
// Module: crate::zero_repeat_side_effects
// Provides: {"array_span_lint"}
// Dependencies: {}
fn array_span_lint (cx : & LateContext < '_ > , expr_span : Span , func_call_span : Span , variable_name_span : Span , expr_ty : Option < Ty < '_ > > , is_vec : bool , is_assign : bool ,) { let has_ty = expr_ty . is_some () ; span_lint_and_sugg (cx , ZERO_REPEAT_SIDE_EFFECTS , expr_span . source_callsite () , "function or method calls as the initial value in zero-sized array initializers may cause side effects" , "consider using" , format ! ("{}; {}{}{} = {}[]{}{}" , snippet (cx , func_call_span . source_callsite () , "..") , if has_ty && ! is_assign { "let " } else { "" } , snippet (cx , variable_name_span . source_callsite () , "..") , if let Some (ty) = expr_ty && ! is_assign { format ! (": {ty}") } else { String :: new () } , if is_vec { "vec!" } else { "" } , if let Some (ty) = expr_ty && is_assign { format ! (" as {ty}") } else { String :: new () } , if is_assign { "" } else { ";" }) , Applicability :: Unspecified ,) ; }
};
}
