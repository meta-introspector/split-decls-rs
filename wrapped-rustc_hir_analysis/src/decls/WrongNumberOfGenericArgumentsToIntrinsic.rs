macro_rules! WrongNumberOfGenericArgumentsToIntrinsic {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_wrong_number_of_generic_arguments_to_intrinsic , code = E0094)] pub (crate) struct WrongNumberOfGenericArgumentsToIntrinsic < 'a > { # [primary_span] # [label] pub span : Span , pub found : usize , pub expected : usize , pub descr : & 'a str , }
    };
}

WrongNumberOfGenericArgumentsToIntrinsic!()