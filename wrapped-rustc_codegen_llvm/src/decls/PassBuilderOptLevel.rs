macro_rules! PassBuilderOptLevel {
    () => {
        # [doc = " LLVMRustPassBuilderOptLevel"] # [repr (C)] pub (crate) enum PassBuilderOptLevel { O0 , O1 , O2 , O3 , Os , Oz , }
    };
}

PassBuilderOptLevel!()