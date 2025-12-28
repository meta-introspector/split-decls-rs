macro_rules! deps {
    () => {
        AssocItemLoc!();
    };
}

macro_rules! StaticLoc {
    () => {
        deps!();
        pub type StaticLoc = AssocItemLoc < ast :: Static > ;
    };
}

StaticLoc!()