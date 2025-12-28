macro_rules! InputFileWouldBeOverWritten {
    () => {
        # [derive (Diagnostic)] # [diag (interface_input_file_would_be_overwritten)] pub struct InputFileWouldBeOverWritten < 'a > { pub path : & 'a Path , }
    };
}

InputFileWouldBeOverWritten!();