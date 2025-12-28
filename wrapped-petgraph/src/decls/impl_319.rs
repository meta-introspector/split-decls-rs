macro_rules! deps {
    () => {
        List!();
        IndexType!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl < E , Ix : IndexType > visit :: NodeCompactIndexable for List < E , Ix > { }
    };
}

impl_319!();