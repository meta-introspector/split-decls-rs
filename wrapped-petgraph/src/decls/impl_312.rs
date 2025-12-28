macro_rules! deps {
    () => {
        IndexType!();
        EdgeReference!();
        List!();
        EdgeReferences!();
        EdgeRef!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl < 'a , Ix : IndexType , E > visit :: IntoEdgeReferences for & 'a List < E , Ix > { type EdgeRef = EdgeReference < 'a , E , Ix > ; type EdgeReferences = EdgeReferences < 'a , E , Ix > ; fn edge_references (self) -> Self :: EdgeReferences { let iter = self . suc . iter () . enumerate () . flat_map (proj2 as _) ; EdgeReferences { iter } } }
    };
}

impl_312!();