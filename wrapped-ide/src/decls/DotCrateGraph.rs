macro_rules! DotCrateGraph {
    () => {
        struct DotCrateGraph < 'db > { crates_to_render : FxHashMap < Crate , (& 'db BuiltCrateData , & 'db ExtraCrateData) > , }
    };
}

DotCrateGraph!();