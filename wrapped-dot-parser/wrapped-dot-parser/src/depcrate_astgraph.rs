// Generated macro for Graph (struct)
macro_rules! Depcrate_astGraph {
() => {
// Module: crate::ast
// Provides: {"Graph"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq , Ord , PartialOrd , Hash , Clone)] # [doc = " This structure is an AST for the DOT graph language.  The generic `A`"] # [doc = " denotes the type of attributes. By default (and when parsing), it is"] # [doc = " `(&'a str, &'a str)`, i.e. two strings, one for the key and one for the"] # [doc = " value of the attribute. The library provides functions to map from one type"] # [doc = " to an other."] pub struct Graph < A > { # [doc = " Specifies if the `Graph` is strict or not. A \"strict\" graph must not"] # [doc = " contain the same edge multiple times. Notice that, for undirected edge,"] # [doc = " an edge from `A` to `B` and an edge from `B` to `A` are equals."] pub strict : bool , # [doc = " Specifies if the `Graph` is directed."] pub is_digraph : bool , # [doc = " The name of the `Graph`, if any."] pub name : Option < String > , # [doc = " The statements that describe the graph."] pub stmts : StmtList < A > , }
};
}
