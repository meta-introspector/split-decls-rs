macro_rules! macro_97 {
    () => {
        trait_template ! { # [doc = " A graph with a known node count."] # [allow (clippy :: needless_arbitrary_self_type)] pub trait NodeCount : GraphBase { @ section self fn node_count (self : & Self) -> usize ; } }
    };
}

macro_97!();