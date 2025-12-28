macro_rules! deps {
    () => {
        ProjectionElem!();
        LocalId!();
    };
}

macro_rules! PlaceElem {
    () => {
        deps!();
        type PlaceElem < 'db > = ProjectionElem < LocalId < 'db > , Ty < 'db > > ;
    };
}

PlaceElem!()