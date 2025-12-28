macro_rules! deps {
    () => {
        Match!();
    };
}

macro_rules! MatchCollector {
    () => {
        deps!();
        # [derive (Default)] struct MatchCollector { matches_by_node : FxHashMap < SyntaxNode , Match > , }
    };
}

MatchCollector!()