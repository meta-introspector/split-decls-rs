macro_rules! hash_bytes {
    () => {
        # [doc = " Hashes some bytes into a 64-bit hash, via LLVM's `IndexedInstrProf::ComputeHash`,"] # [doc = " as required for parts of the LLVM coverage mapping format."] pub (crate) fn hash_bytes (bytes : & [u8]) -> u64 { unsafe { llvm :: LLVMRustCoverageHashBytes (bytes . as_c_char_ptr () , bytes . len ()) } }
    };
}

hash_bytes!()