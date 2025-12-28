macro_rules! deps {
    () => {
        ReversedEdgeReferences!();
        EdgeRef!();
        ReversedEdgeReference!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < I > Iterator for ReversedEdgeReferences < I > where I : Iterator , I :: Item : EdgeRef , { type Item = ReversedEdgeReference < I :: Item > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (ReversedEdgeReference) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_184!()