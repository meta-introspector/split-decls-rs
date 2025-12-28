macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl < 'tcx > LayoutOfHelpers < 'tcx > for CodegenCx < '_ , 'tcx > { # [inline] fn handle_layout_err (& self , err : LayoutError < 'tcx > , span : Span , ty : Ty < 'tcx >) -> ! { if let LayoutError :: SizeOverflow (_) | LayoutError :: ReferencesError (_) = err { self . tcx . dcx () . emit_fatal (Spanned { span , node : err . into_diagnostic () }) } else { self . tcx . dcx () . emit_fatal (ssa_errors :: FailedToGetLayout { span , ty , err }) } } }
    };
}

impl_230!();