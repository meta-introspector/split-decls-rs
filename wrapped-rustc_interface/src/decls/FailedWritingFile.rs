macro_rules! FailedWritingFile {
    () => {
        # [derive (Diagnostic)] # [diag (interface_failed_writing_file)] pub struct FailedWritingFile < 'a > { pub path : & 'a Path , pub error : io :: Error , }
    };
}

FailedWritingFile!();