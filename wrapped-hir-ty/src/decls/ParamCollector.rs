macro_rules! ParamCollector {
    () => {
        struct ParamCollector { params : FxHashSet < TypeOrConstParamId > , }
    };
}

ParamCollector!();