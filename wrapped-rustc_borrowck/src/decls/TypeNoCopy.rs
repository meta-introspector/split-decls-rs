macro_rules! TypeNoCopy {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum TypeNoCopy < 'a , 'tcx > { # [label (borrowck_ty_no_impl_copy)] Label { is_partial_move : bool , ty : Ty < 'tcx > , place : & 'a str , # [primary_span] span : Span , } , # [note (borrowck_ty_no_impl_copy)] Note { is_partial_move : bool , ty : Ty < 'tcx > , place : & 'a str } , }
    };
}

TypeNoCopy!();