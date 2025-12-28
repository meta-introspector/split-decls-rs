macro_rules! deps {
    () => {
        LocationListTable!();
    };
}

macro_rules! macro_762 {
    () => {
        deps!();
        define_id ! (LocationListId , "An identifier for a location list in a `LocationListTable`.") ;
    };
}

macro_762!();