macro_rules! deps {
    () => {
        Visibility!();
    };
}

macro_rules! get_visibility {
    () => {
        deps!();
        pub (crate) fn get_visibility (llglobal : & Value) -> Visibility { unsafe { LLVMGetVisibility (llglobal) } . to_rust () }
    };
}

get_visibility!()