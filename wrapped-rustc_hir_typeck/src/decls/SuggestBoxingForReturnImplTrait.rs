macro_rules! SuggestBoxingForReturnImplTrait {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum SuggestBoxingForReturnImplTrait { # [multipart_suggestion (hir_typeck_rpit_change_return_type , applicability = "maybe-incorrect")] ChangeReturnType { # [suggestion_part (code = "Box<dyn")] start_sp : Span , # [suggestion_part (code = ">")] end_sp : Span , } , # [multipart_suggestion (hir_typeck_rpit_box_return_expr , applicability = "maybe-incorrect")] BoxReturnExpr { # [suggestion_part (code = "Box::new(")] starts : Vec < Span > , # [suggestion_part (code = ")")] ends : Vec < Span > , } , }
    };
}

SuggestBoxingForReturnImplTrait!();