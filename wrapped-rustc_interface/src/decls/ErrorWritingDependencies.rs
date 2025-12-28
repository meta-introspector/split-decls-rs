macro_rules! ErrorWritingDependencies {
    () => {
        # [derive (Diagnostic)] # [diag (interface_error_writing_dependencies)] pub struct ErrorWritingDependencies < 'a > { pub path : & 'a Path , pub error : io :: Error , }
    };
}

ErrorWritingDependencies!();