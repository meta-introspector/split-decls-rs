macro_rules! TraitBoundConst {
    () => {
        enum TraitBoundConst { None , # [cfg (feature = "verbatim")] Conditional , # [cfg (feature = "verbatim")] Unconditional , }
    };
}

TraitBoundConst!()