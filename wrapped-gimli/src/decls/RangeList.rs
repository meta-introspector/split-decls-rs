macro_rules! deps {
    () => {
        Range!();
    };
}

macro_rules! RangeList {
    () => {
        deps!();
        # [doc = " A range list that will be stored in a `.debug_ranges` or `.debug_rnglists` section."] # [derive (Clone , Debug , Eq , PartialEq , Hash)] pub struct RangeList (pub Vec < Range >) ;
    };
}

RangeList!()