macro_rules! DiffBinaryFile {
    () => {
        # [doc = " The contents of one of the files in a binary diff."] pub struct DiffBinaryFile < 'a > { raw : * const raw :: git_diff_binary_file , _marker : marker :: PhantomData < & 'a raw :: git_diff_binary_file > , }
    };
}

DiffBinaryFile!();