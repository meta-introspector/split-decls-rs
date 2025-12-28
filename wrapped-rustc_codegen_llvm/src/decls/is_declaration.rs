macro_rules! is_declaration {
    () => {
        pub (crate) fn is_declaration (llglobal : & Value) -> bool { unsafe { LLVMIsDeclaration (llglobal) } . is_true () }
    };
}

is_declaration!();