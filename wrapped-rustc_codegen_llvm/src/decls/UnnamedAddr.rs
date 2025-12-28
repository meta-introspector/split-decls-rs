macro_rules! UnnamedAddr {
    () => {
        # [doc = " LLVMUnnamedAddr"] # [repr (C)] pub (crate) enum UnnamedAddr { No , # [expect (dead_code)] Local , Global , }
    };
}

UnnamedAddr!();