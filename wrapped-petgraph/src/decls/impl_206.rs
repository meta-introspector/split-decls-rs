macro_rules! deps {
    () => {
        EdgeRef!();
        MaybeReversedEdgeReferences!();
        MaybeReversedEdgeReference!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < I > Iterator for MaybeReversedEdgeReferences < I > where I : Iterator , I :: Item : EdgeRef , { type Item = MaybeReversedEdgeReference < I :: Item > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| x | MaybeReversedEdgeReference { inner : x , reversed : false , }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_206!()