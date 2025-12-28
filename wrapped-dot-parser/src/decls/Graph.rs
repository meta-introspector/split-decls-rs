macro_rules! deps {
    () => {
        StmtList!();
    };
}

macro_rules! Graph {
    () => {
        deps!();
        # [doc = " A graph, very similar to Ast::Graph, but can not contain subgraphs."] pub struct Graph < A > { # [doc = " Specifies if the `Graph` is strict or not. A \"strict\" graph must not"] # [doc = " contain the same edge multiple times. Notice that, for undirected edge,"] # [doc = " an edge from `A` to `B` and an edge from `B` to `A` are equals."] pub strict : bool , # [doc = " Specifies if the `Graph` is directed."] pub is_digraph : bool , # [doc = " The name of the `Graph`, if any."] pub name : Option < String > , # [doc = " The statements that describe the graph."] pub stmts : StmtList < A > , }
    };
}

Graph!()