macro_rules! deps {
    () => {
        EdgeType!();
        Nullable!();
        Neighbors!();
        NodeIndex!();
        IndexType!();
    };
}

macro_rules! impl_1001 {
    () => {
        deps!();
        impl < Ty : EdgeType , Null : Nullable , Ix : IndexType > Iterator for Neighbors < '_ , Ty , Null , Ix > { type Item = NodeIndex < Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (| (_ , b , _) | b) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
    };
}

impl_1001!();