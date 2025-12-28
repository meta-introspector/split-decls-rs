macro_rules! deps {
    () => {
        IndexType!();
        List!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl < E , Ix : IndexType > visit :: Data for List < E , Ix > { type NodeWeight = () ; type EdgeWeight = E ; }
    };
}

impl_305!()