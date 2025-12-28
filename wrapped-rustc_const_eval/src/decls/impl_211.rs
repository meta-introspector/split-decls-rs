macro_rules! deps {
    () => {
        Machine!();
        InterpCx!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl < 'tcx , M : Machine < 'tcx > > FnAbiOfHelpers < 'tcx > for InterpCx < 'tcx , M > { type FnAbiOfResult = Result < & 'tcx FnAbi < 'tcx , Ty < 'tcx > > , InterpErrorKind < 'tcx > > ; fn handle_fn_abi_err (& self , err : FnAbiError < 'tcx > , _span : Span , _fn_abi_request : FnAbiRequest < 'tcx > ,) -> InterpErrorKind < 'tcx > { match err { FnAbiError :: Layout (err) => err_inval ! (Layout (err)) , } } }
    };
}

impl_211!();