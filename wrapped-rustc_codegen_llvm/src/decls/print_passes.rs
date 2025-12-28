macro_rules! print_passes {
    () => {
        pub (crate) fn print_passes () { unsafe { llvm :: LLVMRustPrintPasses () ; } }
    };
}

print_passes!()