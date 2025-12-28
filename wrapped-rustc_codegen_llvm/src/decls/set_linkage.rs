macro_rules! deps {
    () => {
        Linkage!();
    };
}

macro_rules! set_linkage {
    () => {
        deps!();
        pub (crate) fn set_linkage (llglobal : & Value , linkage : Linkage) { unsafe { LLVMSetLinkage (llglobal , linkage) ; } }
    };
}

set_linkage!()