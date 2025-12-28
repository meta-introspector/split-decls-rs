macro_rules! deps {
    () => {
        Cache!();
    };
}

macro_rules! BlockBasedOptionsMustOutliveDB {
    () => {
        deps!();
        # [derive (Default)] struct BlockBasedOptionsMustOutliveDB { block_cache : Option < Cache > , }
    };
}

BlockBasedOptionsMustOutliveDB!()