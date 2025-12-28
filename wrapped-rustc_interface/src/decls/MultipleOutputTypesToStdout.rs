macro_rules! MultipleOutputTypesToStdout {
    () => {
        # [derive (Diagnostic)] # [diag (interface_multiple_output_types_to_stdout)] pub struct MultipleOutputTypesToStdout ;
    };
}

MultipleOutputTypesToStdout!();