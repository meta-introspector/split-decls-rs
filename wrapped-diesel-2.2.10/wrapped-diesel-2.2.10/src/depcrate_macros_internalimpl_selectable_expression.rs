// Generated macro for impl_selectable_expression (macro)
macro_rules! Depcrate_macros_internalimpl_selectable_expression {
() => {
// Module: crate::macros::internal
// Provides: {"impl_selectable_expression"}
// Dependencies: {}
# [doc = " This will implement `SelectableExpression` and `AppearsOnTable` for \"simple\""] # [doc = " composite nodes where the where clause is roughly `AllTyParams:"] # [doc = " SelectableExpression<QS>, Self: Expression`."] # [doc = ""] # [doc = " This macro is exported because we want to be able to call it from other"] # [doc = " macros that are exported, but it is not part of our public API."] # [macro_export] # [doc (hidden)] macro_rules ! impl_selectable_expression { ($ struct_name : ident) => { $ crate :: impl_selectable_expression ! (ty_params = () , struct_ty = $ struct_name ,) ; } ; ($ struct_name : ident <$ ($ ty_params : ident) ,+>) => { $ crate :: impl_selectable_expression ! (ty_params = ($ ($ ty_params) ,+) , struct_ty = $ struct_name <$ ($ ty_params) ,+>,) ; } ; (ty_params = ($ ($ ty_params : ident) ,*) , struct_ty = $ struct_ty : ty ,) => { impl <$ ($ ty_params ,) * QS > $ crate :: expression :: SelectableExpression < QS > for $ struct_ty where $ struct_ty : $ crate :: expression :: AppearsOnTable < QS >, $ ($ ty_params : $ crate :: expression :: SelectableExpression < QS >,) * { } impl <$ ($ ty_params ,) * QS > $ crate :: expression :: AppearsOnTable < QS > for $ struct_ty where $ struct_ty : $ crate :: expression :: Expression , $ ($ ty_params : $ crate :: expression :: AppearsOnTable < QS >,) * { } } ; }
};
}
