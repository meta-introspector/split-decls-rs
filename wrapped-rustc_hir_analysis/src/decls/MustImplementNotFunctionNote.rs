macro_rules! MustImplementNotFunctionNote {
    () => {
        # [derive (Subdiagnostic)] # [note (hir_analysis_must_implement_not_function_note)] pub (crate) struct MustImplementNotFunctionNote { }
    };
}

MustImplementNotFunctionNote!()