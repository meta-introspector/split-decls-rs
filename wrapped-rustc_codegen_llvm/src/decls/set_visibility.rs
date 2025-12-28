macro_rules! deps {
    () => {
        Visibility!();
    };
}

macro_rules! set_visibility {
    () => {
        deps!();
        pub (crate) fn set_visibility (llglobal : & Value , visibility : Visibility) { unsafe { LLVMSetVisibility (llglobal , visibility) ; } }
    };
}

set_visibility!()