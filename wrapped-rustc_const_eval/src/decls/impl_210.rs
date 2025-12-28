macro_rules! deps {
    () => {
        InterpCx!();
        Machine!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl < 'tcx , M : Machine < 'tcx > > LayoutOfHelpers < 'tcx > for InterpCx < 'tcx , M > { type LayoutOfResult = Result < TyAndLayout < 'tcx > , InterpErrorKind < 'tcx > > ; # [inline] fn layout_tcx_at_span (& self) -> Span { self . tcx . span } # [inline] fn handle_layout_err (& self , err : LayoutError < 'tcx > , _ : Span , _ : Ty < 'tcx > ,) -> InterpErrorKind < 'tcx > { err_inval ! (Layout (err)) } }
    };
}

impl_210!()