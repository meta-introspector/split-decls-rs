macro_rules! deps {
    () => {
        BaseExpressionDoubleDotAddExpr!();
        BaseExpressionDoubleDotRemove!();
    };
}

macro_rules! BaseExpressionDoubleDot {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_typeck_base_expression_double_dot , code = E0797)] pub (crate) struct BaseExpressionDoubleDot { # [primary_span] pub span : Span , # [suggestion (hir_typeck_base_expression_double_dot_enable_default_field_values , code = "#![feature(default_field_values)]\n" , applicability = "machine-applicable" , style = "verbose")] pub default_field_values_suggestion : Option < Span > , # [subdiagnostic] pub add_expr : Option < BaseExpressionDoubleDotAddExpr > , # [subdiagnostic] pub remove_dots : Option < BaseExpressionDoubleDotRemove > , }
    };
}

BaseExpressionDoubleDot!()