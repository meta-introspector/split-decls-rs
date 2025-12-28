macro_rules! NamedArgumentUsedPositionally {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_named_argument_used_positionally)] pub (crate) struct NamedArgumentUsedPositionally { # [label (lint_label_named_arg)] pub named_arg_sp : Span , # [label (lint_label_position_arg)] pub position_label_sp : Option < Span > , # [suggestion (style = "verbose" , code = "{name}" , applicability = "maybe-incorrect")] pub suggestion : Option < Span > , pub name : String , pub named_arg_name : String , }
    };
}

NamedArgumentUsedPositionally!();