macro_rules! deps {
    () => {
        MinInt!();
    };
}

macro_rules! OtherSign {
    () => {
        deps!();
        # [doc = " Access the associated `OtherSign` type from an int (helper to avoid ambiguous associated"] # [doc = " types)."] pub type OtherSign < I > = < I as MinInt > :: OtherSign ;
    };
}

OtherSign!()