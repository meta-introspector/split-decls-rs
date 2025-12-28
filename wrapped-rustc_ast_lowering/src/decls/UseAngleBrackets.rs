macro_rules! UseAngleBrackets {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (ast_lowering_use_angle_brackets , applicability = "maybe-incorrect")] pub (crate) struct UseAngleBrackets { # [suggestion_part (code = "<")] pub open_param : Span , # [suggestion_part (code = ">")] pub close_param : Span , }
    };
}

UseAngleBrackets!();