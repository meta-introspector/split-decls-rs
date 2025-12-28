macro_rules! deps {
    () => {
        ReversedEdgeReference!();
        ReversedEdges!();
        EdgeRef!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < I > Iterator for ReversedEdges < I > where I : Iterator , I :: Item : EdgeRef , { type Item = ReversedEdgeReference < I :: Item > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (ReversedEdgeReference) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_178!()