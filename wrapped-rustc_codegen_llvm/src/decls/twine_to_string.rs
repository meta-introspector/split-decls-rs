macro_rules! twine_to_string {
    () => {
        pub (crate) fn twine_to_string (tr : & Twine) -> String { unsafe { build_string (| s | LLVMRustWriteTwineToString (tr , s)) . expect ("got a non-UTF8 Twine from LLVM") } }
    };
}

twine_to_string!()