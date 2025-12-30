// Generated macro for numeric_expr (macro)
macro_rules! Depcrate_macros_opsnumeric_expr {
() => {
// Module: crate::macros::ops
// Provides: {"numeric_expr"}
// Dependencies: {}
# [macro_export] # [doc = " Indicates that an expression allows all numeric operators. If you create new"] # [doc = " SQL functions that return a numeric type, you should invoke this macro that"] # [doc = " type. Unfortunately, Rust disallows us from automatically implementing `Add`"] # [doc = " for types which implement `Expression`, under its orphan rules."] macro_rules ! numeric_expr { ($ tpe : ty) => { $ crate :: operator_allowed ! ($ tpe , Add , add) ; $ crate :: operator_allowed ! ($ tpe , Sub , sub) ; $ crate :: operator_allowed ! ($ tpe , Div , div) ; $ crate :: operator_allowed ! ($ tpe , Mul , mul) ; } ; }
};
}
