macro_rules! ThinLTOKeysMap {
    () => {
        # [doc = " Maps LLVM module identifiers to their corresponding LLVM LTO cache keys"] # [derive (Debug , Default)] struct ThinLTOKeysMap { keys : BTreeMap < String , String > , }
    };
}

ThinLTOKeysMap!();