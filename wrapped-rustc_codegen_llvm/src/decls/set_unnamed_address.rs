macro_rules! deps {
    () => {
        UnnamedAddr!();
    };
}

macro_rules! set_unnamed_address {
    () => {
        deps!();
        pub (crate) fn set_unnamed_address (global : & Value , unnamed : UnnamedAddr) { LLVMSetUnnamedAddress (global , unnamed) ; }
    };
}

set_unnamed_address!()