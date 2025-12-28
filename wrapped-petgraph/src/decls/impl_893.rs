macro_rules! deps {
    () => {
        NodeTrait!();
        EdgeType!();
        Neighbors!();
    };
}

macro_rules! impl_893 {
    () => {
        deps!();
        impl < N , Ty > Iterator for Neighbors < '_ , N , Ty > where N : NodeTrait , Ty : EdgeType , { type Item = N ; fn next (& mut self) -> Option < N > { if Ty :: is_directed () { (& mut self . iter) . filter_map (| & (n , dir) | if dir == Outgoing { Some (n) } else { None }) . next () } else { self . iter . next () . map (| & (n , _) | n) } } fn size_hint (& self) -> (usize , Option < usize >) { let (lower , upper) = self . iter . size_hint () ; if Ty :: is_directed () { (0 , upper) } else { (lower , upper) } } }
    };
}

impl_893!()