macro_rules! write_filenames_to_buffer {
    () => {
        pub (crate) fn write_filenames_to_buffer (filenames : & [impl AsRef < str >]) -> Vec < u8 > { let (pointers , lengths) = filenames . into_iter () . map (AsRef :: as_ref) . map (| s : & str | (s . as_c_char_ptr () , s . len ())) . unzip :: < _ , _ , Vec < _ > , Vec < _ > > () ; llvm :: build_byte_buffer (| buffer | unsafe { llvm :: LLVMRustCoverageWriteFilenamesToBuffer (pointers . as_ptr () , pointers . len () , lengths . as_ptr () , lengths . len () , buffer ,) ; }) }
    };
}

write_filenames_to_buffer!();