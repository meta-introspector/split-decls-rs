macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < 'tcx > LayoutOfHelpers < 'tcx > for Builder < '_ , '_ , 'tcx > { # [inline] fn handle_layout_err (& self , err : LayoutError < 'tcx > , span : Span , ty : Ty < 'tcx >) -> ! { self . cx . handle_layout_err (err , span , ty) } }
    };
}

impl_167!();