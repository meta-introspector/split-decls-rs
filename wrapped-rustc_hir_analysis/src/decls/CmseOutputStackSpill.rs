macro_rules! CmseOutputStackSpill {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_cmse_output_stack_spill , code = E0798)] # [note (hir_analysis_note1)] # [note (hir_analysis_note2)] pub (crate) struct CmseOutputStackSpill { # [primary_span] # [label] pub span : Span , pub abi : ExternAbi , }
    };
}

CmseOutputStackSpill!()