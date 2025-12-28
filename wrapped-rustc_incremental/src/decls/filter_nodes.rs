macro_rules! filter_nodes {
    () => {
        fn filter_nodes < 'q > (query : & 'q DepGraphQuery , sources : & Option < FxIndexSet < & 'q DepNode > > , targets : & Option < FxIndexSet < & 'q DepNode > > ,) -> FxIndexSet < DepKind > { if let Some (sources) = sources { if let Some (targets) = targets { walk_between (query , sources , targets) } else { walk_nodes (query , sources , OUTGOING) } } else if let Some (targets) = targets { walk_nodes (query , targets , INCOMING) } else { query . nodes () . into_iter () . map (| n | n . kind) . collect () } }
    };
}

filter_nodes!()