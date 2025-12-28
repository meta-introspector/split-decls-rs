macro_rules! deps {
    () => {
        NodeIndex!();
        Neighbors!();
        IndexType!();
    };
}

macro_rules! impl_537 {
    () => {
        deps!();
        impl < Ix > Iterator for Neighbors < '_ , Ix > where Ix : IndexType , { type Item = NodeIndex < Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . cloned () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_537!()