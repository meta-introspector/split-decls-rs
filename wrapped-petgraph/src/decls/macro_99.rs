macro_rules! macro_99 {
    () => {
        trait_template ! { # [doc = " The graph’s `NodeId`s map to indices, in a range without holes."] # [doc = ""] # [doc = " The graph's node identifiers correspond to exactly the indices"] # [doc = " `0..self.node_bound()`."] pub trait NodeCompactIndexable : NodeIndexable + NodeCount { } }
    };
}

macro_99!()