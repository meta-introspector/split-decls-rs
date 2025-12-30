// Generated macro for operator_allowed (macro)
macro_rules! Depcrate_macros_opsoperator_allowed {
() => {
// Module: crate::macros::ops
// Provides: {"operator_allowed"}
// Dependencies: {}
# [macro_export] # [doc = " Implements the Rust operator for a given type. If you create a new SQL"] # [doc = " function, which returns a type that you'd like to use an operator on, you"] # [doc = " should invoke this macro. Unfortunately, Rust disallows us from"] # [doc = " automatically implementing `Add` and other traits from `std::ops`, under its"] # [doc = " orphan rules."] macro_rules ! operator_allowed { ($ tpe : ty , $ op : ident , $ fn_name : ident) => { impl < Rhs > :: std :: ops ::$ op < Rhs > for $ tpe where Rhs : $ crate :: expression :: AsExpression < <<$ tpe as $ crate :: Expression >:: SqlType as $ crate :: sql_types :: ops ::$ op >:: Rhs , >, { type Output = $ crate :: internal :: table_macro :: ops ::$ op < Self , Rhs :: Expression >; fn $ fn_name (self , __diesel_internal_rhs : Rhs) -> Self :: Output { $ crate :: internal :: table_macro :: ops ::$ op :: new (self , __diesel_internal_rhs . as_expression () ,) } } } ; }
};
}
