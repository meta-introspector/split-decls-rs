// Generated macro for Graph (struct)
macro_rules! Depcrate_canonicalGraph {
() => {
// Module: crate::canonical
// Provides: {"Graph"}
// Dependencies: {}
# [doc = " A `Graph` is a structure that can be created from a regular"] # [doc = " `Graph`, but that is more friendly to work with. For instance, in a `Graph`,"] # [doc = " attributes are most often given as a list of list of `Attr`, while in a"] # [doc = " `Graph`, the lists are flatten."] # [derive (Debug , Clone)] pub struct Graph < A > { # [doc = " Specifies if the `Graph` is strict or not. A \"strict\" graph must not"] # [doc = " contain the same edge multiple times. Notice that, for undirected edge,"] # [doc = " an edge from `A` to `B` and an edge from `B` to `A` are equals."] pub strict : bool , # [doc = " Specifies if the `Graph` is directed."] pub is_digraph : bool , # [doc = " The name of the `Graph`, if any."] pub name : Option < String > , # [doc = " The global attributes of the graph."] pub attr : Vec < AttrStmt < A > > , # [doc = " The nodes of the graph."] pub nodes : NodeSet < A > , # [doc = " The edges of the graph."] pub edges : EdgeSet < A > , # [doc = " The ID equalities declared in the graph."] pub ideqs : Vec < IDEq > , }
};
}
