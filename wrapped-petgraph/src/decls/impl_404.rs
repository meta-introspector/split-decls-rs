macro_rules! deps {
    () => {
        MatchedNodes!();
    };
}

macro_rules! impl_404 {
    () => {
        deps!();
        impl < G > Iterator for MatchedNodes < '_ , G > where G : NodeIndexable , { type Item = G :: NodeId ; fn next (& mut self) -> Option < Self :: Item > { while self . current != self . mate . len () { let current = self . current ; self . current += 1 ; if self . mate [current] . is_some () { return Some (self . graph . from_index (current)) ; } } None } }
    };
}

impl_404!()