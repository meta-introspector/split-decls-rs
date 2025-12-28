macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < 'tcx > LayoutOfHelpers < 'tcx > for LateContext < 'tcx > { type LayoutOfResult = Result < TyAndLayout < 'tcx > , LayoutError < 'tcx > > ; # [inline] fn handle_layout_err (& self , err : LayoutError < 'tcx > , _ : Span , _ : Ty < 'tcx >) -> LayoutError < 'tcx > { err } }
    };
}

impl_133!()