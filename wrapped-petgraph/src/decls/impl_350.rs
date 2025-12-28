macro_rules! deps {
    () => {
        DominatorsIter!();
        DominatedByIter!();
        Dominators!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl < N > Dominators < N > where N : Copy + Eq + Hash , { # [doc = " Get the root node used to construct these dominance relations."] pub fn root (& self) -> N { self . root } # [doc = " Get the immediate dominator of the given node."] # [doc = ""] # [doc = " Returns `None` for any node that is not reachable from the root, and for"] # [doc = " the root itself."] pub fn immediate_dominator (& self , node : N) -> Option < N > { if node == self . root { None } else { self . dominators . get (& node) . cloned () } } # [doc = " Iterate over the given node's strict dominators."] # [doc = ""] # [doc = " If the given node is not reachable from the root, then `None` is"] # [doc = " returned."] pub fn strict_dominators (& self , node : N) -> Option < DominatorsIter < '_ , N > > { if self . dominators . contains_key (& node) { Some (DominatorsIter { dominators : self , node : self . immediate_dominator (node) , }) } else { None } } # [doc = " Iterate over all of the given node's dominators (including the given"] # [doc = " node itself)."] # [doc = ""] # [doc = " If the given node is not reachable from the root, then `None` is"] # [doc = " returned."] pub fn dominators (& self , node : N) -> Option < DominatorsIter < '_ , N > > { if self . dominators . contains_key (& node) { Some (DominatorsIter { dominators : self , node : Some (node) , }) } else { None } } # [doc = " Iterate over all nodes immediately dominated by the given node (not"] # [doc = " including the given node itself)."] pub fn immediately_dominated_by (& self , node : N) -> DominatedByIter < '_ , N > { DominatedByIter { iter : self . dominators . iter () , node , } } }
    };
}

impl_350!()