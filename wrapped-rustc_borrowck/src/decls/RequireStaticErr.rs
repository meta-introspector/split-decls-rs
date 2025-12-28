macro_rules! RequireStaticErr {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum RequireStaticErr { # [note (borrowck_used_impl_require_static)] UsedImpl { # [primary_span] multi_span : MultiSpan , } , }
    };
}

RequireStaticErr!()