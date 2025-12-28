macro_rules! deps {
    () => {
        Linkage!();
    };
}

macro_rules! get_linkage {
    () => {
        deps!();
        pub (crate) fn get_linkage (llglobal : & Value) -> Linkage { unsafe { LLVMGetLinkage (llglobal) } . to_rust () }
    };
}

get_linkage!();