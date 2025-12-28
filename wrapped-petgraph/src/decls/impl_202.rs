macro_rules! deps {
    () => {
        MaybeReversedEdgeReference!();
        EdgeRef!();
        MaybeReversedEdges!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl < I > Iterator for MaybeReversedEdges < I > where I : Iterator , I :: Item : EdgeRef , { type Item = MaybeReversedEdgeReference < I :: Item > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| x | MaybeReversedEdgeReference { inner : x , reversed : self . reversed , }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_202!();