mkuse!{use super :: * ;}
mkitem!{mkimpl!{impl < 'graph , G : DirectedGraph > DirectedGraph for & 'graph G { type Node = G :: Node ; fn num_nodes (& self) -> usize { (* * self) . num_nodes () } }}}
mkitem!{mkimpl!{impl < 'graph , G : StartNode > StartNode for & 'graph G { fn start_node (& self) -> Self :: Node { (* * self) . start_node () } }}}
mkitem!{mkimpl!{impl < 'graph , G : Successors > Successors for & 'graph G { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { (* * self) . successors (node) } }}}
mkitem!{mkimpl!{impl < 'graph , G : Predecessors > Predecessors for & 'graph G { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { (* * self) . predecessors (node) } }}}