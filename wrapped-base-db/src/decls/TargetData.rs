macro_rules! deps {
    () => {
        Arch!();
    };
}

macro_rules! TargetData {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq , Hash , Clone)] pub struct TargetData { pub data_layout : Box < str > , pub arch : Arch , }
    };
}

TargetData!();