macro_rules! should_emit_generic_error {
    () => {
        fn should_emit_generic_error < 'tcx > (abi : ExternAbi , layout_err : & 'tcx LayoutError < 'tcx >) -> bool { use LayoutError :: * ; match layout_err { TooGeneric (ty) => { match abi { ExternAbi :: CmseNonSecureCall => { ! ty . is_impl_trait () } ExternAbi :: CmseNonSecureEntry => true , _ => bug ! ("invalid ABI: {abi}") , } } Unknown (..) | SizeOverflow (..) | NormalizationFailure (..) | ReferencesError (..) | Cycle (..) => { false } } }
    };
}

should_emit_generic_error!()