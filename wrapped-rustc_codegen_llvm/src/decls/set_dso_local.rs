macro_rules! set_dso_local {
    () => {
        pub (crate) fn set_dso_local < 'll > (v : & 'll Value) { unsafe { LLVMRustSetDSOLocal (v , true) ; } }
    };
}

set_dso_local!();