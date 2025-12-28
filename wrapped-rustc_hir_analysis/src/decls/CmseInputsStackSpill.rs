macro_rules! CmseInputsStackSpill {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_cmse_inputs_stack_spill , code = E0798)] # [note] pub (crate) struct CmseInputsStackSpill { # [primary_span] # [label] pub span : Span , pub plural : bool , pub abi : ExternAbi , }
    };
}

CmseInputsStackSpill!();