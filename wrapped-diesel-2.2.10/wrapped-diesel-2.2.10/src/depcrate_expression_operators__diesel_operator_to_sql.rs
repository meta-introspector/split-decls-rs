// Generated macro for __diesel_operator_to_sql (macro)
macro_rules! Depcrate_expression_operators__diesel_operator_to_sql {
() => {
// Module: crate::expression::operators
// Provides: {"__diesel_operator_to_sql"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! __diesel_operator_to_sql { (notation = infix , operator_expr = $ op : expr , field_exprs = ($ left : expr , $ right : expr) ,) => { $ left ; $ op ; $ right ; } ; (notation = postfix , operator_expr = $ op : expr , field_exprs = ($ expr : expr) ,) => { $ expr ; $ op ; } ; (notation = prefix , operator_expr = $ op : expr , field_exprs = ($ expr : expr) ,) => { $ op ; $ expr ; } ; }
};
}
