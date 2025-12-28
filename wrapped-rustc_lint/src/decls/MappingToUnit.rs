macro_rules! MappingToUnit {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_map_unit_fn)] # [note] pub (crate) struct MappingToUnit { # [label (lint_function_label)] pub function_label : Span , # [label (lint_argument_label)] pub argument_label : Span , # [label (lint_map_label)] pub map_label : Span , # [suggestion (style = "verbose" , code = "for_each" , applicability = "maybe-incorrect")] pub suggestion : Span , }
    };
}

MappingToUnit!()