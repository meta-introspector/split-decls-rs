// Generated macro for postfix_operator (macro)
macro_rules! Depcrate_expression_operatorspostfix_operator {
() => {
// Module: crate::expression::operators
// Provides: {"postfix_operator"}
// Dependencies: {}
# [doc = " Useful for libraries adding support for new SQL types. Apps should never"] # [doc = " need to call this."] # [doc = ""] # [doc = " Similar to [`infix_operator!`], but the generated type will only take"] # [doc = " a single argument rather than two. The operator SQL will be placed after"] # [doc = " the single argument. See [`infix_operator!`] for example usage."] # [doc = ""] # [macro_export] macro_rules ! postfix_operator { ($ name : ident , $ operator : expr) => { $ crate :: postfix_operator ! ($ name , $ operator , $ crate :: sql_types :: Bool) ; } ; ($ name : ident , $ operator : expr , backend : $ backend : ty) => { $ crate :: postfix_operator ! ($ name , $ operator , $ crate :: sql_types :: Bool , backend : $ backend) ; } ; ($ name : ident , $ operator : expr , $ return_ty : ty) => { $ crate :: __diesel_operator_body ! (notation = postfix , struct_name = $ name , operator = $ operator , return_ty = ($ return_ty) , ty_params = (Expr ,) , field_names = (expr ,) , backend_ty_params = (DB ,) , backend_ty = DB ,) ; } ; ($ name : ident , $ operator : expr , $ return_ty : ty , backend : $ backend : ty) => { $ crate :: __diesel_operator_body ! (notation = postfix , struct_name = $ name , operator = $ operator , return_ty = ($ return_ty) , ty_params = (Expr ,) , field_names = (expr ,) , backend_ty_params = () , backend_ty = $ backend ,) ; } ; }
};
}
