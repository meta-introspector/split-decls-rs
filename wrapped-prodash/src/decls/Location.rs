macro_rules! deps {
    () => {
        Throughput!();
    };
}

macro_rules! Location {
    () => {
        deps!();
        # [doc = " The location at which [`Throughput`] or [`UnitDisplays`][UnitDisplay] should be placed."] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Debug , Hash)] # [allow (missing_docs)] pub enum Location { BeforeValue , AfterUnit , }
    };
}

Location!();