macro_rules! filter_edges {
    () => {
        fn filter_edges (query : & DepGraphQuery , nodes : & FxIndexSet < DepKind >) -> Vec < (DepKind , DepKind) > { let uniq : FxIndexSet < _ > = query . edges () . into_iter () . map (| (s , t) | (s . kind , t . kind)) . filter (| (source , target) | nodes . contains (source) && nodes . contains (target)) . collect () ; uniq . into_iter () . collect () }
    };
}

filter_edges!();