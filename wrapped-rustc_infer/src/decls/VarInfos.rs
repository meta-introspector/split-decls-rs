macro_rules! deps {
    () => {
        RegionVariableInfo!();
    };
}

macro_rules! VarInfos {
    () => {
        deps!();
        pub type VarInfos = IndexVec < RegionVid , RegionVariableInfo > ;
    };
}

VarInfos!()