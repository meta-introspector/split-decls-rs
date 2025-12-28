macro_rules! deps {
    () => {
        Edges!();
        IndexType!();
        EdgeType!();
        EdgeReference!();
    };
}

macro_rules! impl_528 {
    () => {
        deps!();
        impl < 'a , E , Ty , Ix > Iterator for Edges < 'a , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Item = EdgeReference < 'a , E , Ty , Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (move | (& j , w) | { let index = self . index ; self . index += 1 ; EdgeReference { index , source : self . source , target : j , weight : w , ty : PhantomData , } }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_528!();