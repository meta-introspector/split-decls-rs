macro_rules! deps {
    () => {
        Walkable!();
        RangeSyntax!();
    };
}

macro_rules! RangeEnd {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum RangeEnd { # [doc = " `..=` or `...`"] Included (RangeSyntax) , # [doc = " `..`"] Excluded , }
    };
}

RangeEnd!();