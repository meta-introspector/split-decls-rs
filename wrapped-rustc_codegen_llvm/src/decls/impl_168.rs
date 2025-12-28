macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < 'tcx > FnAbiOfHelpers < 'tcx > for Builder < '_ , '_ , 'tcx > { # [inline] fn handle_fn_abi_err (& self , err : FnAbiError < 'tcx > , span : Span , fn_abi_request : FnAbiRequest < 'tcx > ,) -> ! { self . cx . handle_fn_abi_err (err , span , fn_abi_request) } }
    };
}

impl_168!();