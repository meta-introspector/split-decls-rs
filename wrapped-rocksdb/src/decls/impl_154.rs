macro_rules! deps {
    () => {
        BlockBasedOptionsMustOutliveDB!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl BlockBasedOptionsMustOutliveDB { fn clone (& self) -> Self { Self { block_cache : self . block_cache . clone () , } } }
    };
}

impl_154!();