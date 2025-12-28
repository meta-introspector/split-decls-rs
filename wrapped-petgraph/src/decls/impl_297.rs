macro_rules! deps {
    () => {
        EdgeIndex!();
        WSuc!();
        Build!();
        IndexType!();
        List!();
        NodeIndex!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl < E , Ix : IndexType > Build for List < E , Ix > { # [doc = " Adds a new node to the list. This allocates a new `Vec` and then should"] # [doc = " run in amortized **O(1)** time."] fn add_node (& mut self , _weight : ()) -> NodeIndex < Ix > { self . add_node () } # [doc = " Add an edge from `a` to `b` to the graph, with its associated"] # [doc = " data `weight`."] # [doc = ""] # [doc = " Return the index of the new edge."] # [doc = ""] # [doc = " Computes in **O(1)** time."] # [doc = ""] # [doc = " **Panics** if the source node does not exist.<br>"] # [doc = ""] # [doc = " **Note:** `List` allows adding parallel (“duplicate”) edges. If you want"] # [doc = " to avoid this, use [`.update_edge(a, b, weight)`](#method.update_edge) instead."] fn add_edge (& mut self , a : NodeIndex < Ix > , b : NodeIndex < Ix > , weight : E) -> Option < EdgeIndex < Ix > > { Some (self . add_edge (a , b , weight)) } # [doc = " Updates or adds an edge from `a` to `b` to the graph, with its associated"] # [doc = " data `weight`."] # [doc = ""] # [doc = " Return the index of the new edge."] # [doc = ""] # [doc = " Computes in **O(e')** time, where **e'** is the number of successors of `a`."] # [doc = ""] # [doc = " **Panics** if the source node does not exist.<br>"] fn update_edge (& mut self , a : NodeIndex < Ix > , b : NodeIndex < Ix > , weight : E) -> EdgeIndex < Ix > { let row = & mut self . suc [a . index ()] ; for (i , info) in row . iter_mut () . enumerate () { if info . suc == b { info . weight = weight ; return EdgeIndex { from : a , successor_index : i , } ; } } let rank = row . len () ; row . push (WSuc { suc : b , weight }) ; EdgeIndex { from : a , successor_index : rank , } } }
    };
}

impl_297!()