macro_rules! deps {
    () => {
        InterpCx!();
        Machine!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl < 'tcx , M : Machine < 'tcx > > InterpCx < 'tcx , M > { # [doc = " This inherent method takes priority over the trait method with the same name in LayoutOf,"] # [doc = " and allows wrapping the actual [LayoutOf::layout_of] with a tracing span."] # [doc = " See [LayoutOf::layout_of] for the original documentation."] # [inline (always)] pub fn layout_of (& self , ty : Ty < 'tcx >) -> < Self as LayoutOfHelpers < 'tcx > > :: LayoutOfResult { let _trace = enter_trace_span ! (M , layouting :: layout_of , ty = ? ty . kind ()) ; LayoutOf :: layout_of (self , ty) } # [doc = " This inherent method takes priority over the trait method with the same name in FnAbiOf,"] # [doc = " and allows wrapping the actual [FnAbiOf::fn_abi_of_fn_ptr] with a tracing span."] # [doc = " See [FnAbiOf::fn_abi_of_fn_ptr] for the original documentation."] # [inline (always)] pub fn fn_abi_of_fn_ptr (& self , sig : ty :: PolyFnSig < 'tcx > , extra_args : & 'tcx ty :: List < Ty < 'tcx > > ,) -> < Self as FnAbiOfHelpers < 'tcx > > :: FnAbiOfResult { let _trace = enter_trace_span ! (M , layouting :: fn_abi_of_fn_ptr , ? sig , ? extra_args) ; FnAbiOf :: fn_abi_of_fn_ptr (self , sig , extra_args) } # [doc = " This inherent method takes priority over the trait method with the same name in FnAbiOf,"] # [doc = " and allows wrapping the actual [FnAbiOf::fn_abi_of_instance] with a tracing span."] # [doc = " See [FnAbiOf::fn_abi_of_instance] for the original documentation."] # [inline (always)] pub fn fn_abi_of_instance (& self , instance : ty :: Instance < 'tcx > , extra_args : & 'tcx ty :: List < Ty < 'tcx > > ,) -> < Self as FnAbiOfHelpers < 'tcx > > :: FnAbiOfResult { let _trace = enter_trace_span ! (M , layouting :: fn_abi_of_instance , ? instance , ? extra_args) ; FnAbiOf :: fn_abi_of_instance (self , instance , extra_args) } }
    };
}

impl_212!();