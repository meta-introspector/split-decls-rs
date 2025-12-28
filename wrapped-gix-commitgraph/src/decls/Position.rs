macro_rules! deps {
    () => {
        Graph!();
    };
}

macro_rules! Position {
    () => {
        deps!();
        # [doc = " A generalized position for use in [`Graph`]."] # [derive (Clone , Copy , Debug , Eq , Ord , PartialEq , PartialOrd , Hash)] pub struct Position (pub u32) ;
    };
}

Position!()