macro_rules! deps {
    () => {
        Location!();
    };
}

macro_rules! LocationList {
    () => {
        deps!();
        # [doc = " A locations list that will be stored in a `.debug_loc` or `.debug_loclists` section."] # [derive (Clone , Debug , Eq , PartialEq , Hash)] pub struct LocationList (pub Vec < Location >) ;
    };
}

LocationList!();