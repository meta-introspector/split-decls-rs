macro_rules! deps {
    () => {
        DefaultIx!();
        NodeIndex!();
        Row!();
        Directed!();
        Edge!();
    };
}

macro_rules! Csr {
    () => {
        deps!();
        # [doc = " Compressed Sparse Row ([`CSR`]) is a sparse adjacency matrix graph."] # [doc = ""] # [doc = " `CSR` is parameterized over:"] # [doc = ""] # [doc = " - Associated data `N` for nodes and `E` for edges, called *weights*."] # [doc = "   The associated data can be of arbitrary type."] # [doc = " - Edge type `Ty` that determines whether the graph edges are directed or undirected."] # [doc = " - Index type `Ix`, which determines the maximum size of the graph."] # [doc = ""] # [doc = ""] # [doc = " Using **O(|V| + |E|)** space where V is the set of nodes and E is the set of edges."] # [doc = ""] # [doc = " Self loops are allowed, no parallel edges."] # [doc = ""] # [doc = " Fast iteration of the outgoing edges of a node."] # [doc = ""] # [doc = " [`CSR`]: https://en.wikipedia.org/wiki/Sparse_matrix#Compressed_sparse_row_(CSR,_CRS_or_Yale_format)"] # [derive (Debug)] pub struct Csr < N = () , E = () , Ty = Directed , Ix = DefaultIx > { # [doc = " Column of next edge"] column : Vec < NodeIndex < Ix > > , # [doc = " weight of each edge; lock step with column"] edges : Vec < E > , # [doc = " Index of start of row Always node_count + 1 long."] # [doc = " Last element is always equal to column.len()"] row : Vec < usize > , node_weights : Vec < N > , edge_count : usize , ty : PhantomData < Ty > , }
    };
}

Csr!();