macro_rules! RangeSet {
    () => {
        # [doc = " Represents a set of `Size` values as a sorted list of ranges."] # [derive (Debug , Clone)] pub struct RangeSet (Vec < (Size , Size) >) ;
    };
}

RangeSet!()