// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl Kind { # [doc = " The keyword to use to introduce the graph."] # [doc = " Determines which edge syntax must be used, and default style."] fn keyword (& self) -> & 'static str { match * self { Kind :: Digraph => "digraph" , Kind :: Graph => "graph" , } } # [doc = " The edgeop syntax to use for this graph kind."] fn edgeop (& self) -> & 'static str { match * self { Kind :: Digraph => "->" , Kind :: Graph => "--" , } } }
};
}
