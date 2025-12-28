macro_rules! PredecessorSets {
    () => {
        type PredecessorSets < NodeId > = HashMap < NodeId , HashSet < NodeId > > ;
    };
}

PredecessorSets!()