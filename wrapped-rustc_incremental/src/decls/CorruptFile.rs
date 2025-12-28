macro_rules! CorruptFile {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_corrupt_file)] pub (crate) struct CorruptFile < 'a > { pub path : & 'a Path , }
    };
}

CorruptFile!();