macro_rules! deps {
    () => {
        Slope!();
    };
}

macro_rules! Statistic {
    () => {
        deps!();
        # [derive (Clone , Copy , Eq , Ord , PartialEq , PartialOrd , Deserialize , Serialize , Debug)] pub enum Statistic { Mean , Median , MedianAbsDev , Slope , StdDev , Typical , }
    };
}

Statistic!();