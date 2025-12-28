macro_rules! deps {
    () => {
        Binding!();
        DiffBinary!();
        DiffBinaryFile!();
    };
}

macro_rules! impl_354 {
    () => {
        deps!();
        impl < 'a > DiffBinary < 'a > { # [doc = " Returns whether there is data in this binary structure or not."] # [doc = ""] # [doc = " If this is `true`, then this was produced and included binary content."] # [doc = " If this is `false` then this was generated knowing only that a binary"] # [doc = " file changed but without providing the data, probably from a patch that"] # [doc = " said `Binary files a/file.txt and b/file.txt differ`."] pub fn contains_data (& self) -> bool { unsafe { (* self . raw) . contains_data == 1 } } # [doc = " The contents of the old file."] pub fn old_file (& self) -> DiffBinaryFile < 'a > { unsafe { Binding :: from_raw (& (* self . raw) . old_file as * const _) } } # [doc = " The contents of the new file."] pub fn new_file (& self) -> DiffBinaryFile < 'a > { unsafe { Binding :: from_raw (& (* self . raw) . new_file as * const _) } } }
    };
}

impl_354!()