macro_rules! deps {
    () => {
        List!();
        IndexType!();
        Edges!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl < 'a , Ix : IndexType , E > visit :: IntoEdges for & 'a List < E , Ix > { type Edges = OutgoingEdgeReferences < 'a , E , Ix > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { let iter = self . suc [a . index ()] . iter () . enumerate () . zip (core :: iter :: repeat (a)) . map (proj1 as _) ; OutgoingEdgeReferences { iter } } }
    };
}

impl_314!();