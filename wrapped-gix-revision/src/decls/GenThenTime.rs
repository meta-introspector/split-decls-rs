macro_rules! GenThenTime {
    () => {
        # [derive (Debug , Clone , Copy)] struct GenThenTime { # [doc = " Note that the special [`GENERATION_NUMBER_INFINITY`](gix_commitgraph::GENERATION_NUMBER_INFINITY) is used to indicate"] # [doc = " that no commitgraph is available."] generation : gix_revwalk :: graph :: Generation , time : gix_date :: SecondsSinceUnixEpoch , }
    };
}

GenThenTime!()