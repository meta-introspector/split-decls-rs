macro_rules! deps {
    () => {
        CrateGraphBuilder!();
        CrateBuilderId!();
        CrateBuilder!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl ops :: Index < CrateBuilderId > for CrateGraphBuilder { type Output = CrateBuilder ; fn index (& self , index : CrateBuilderId) -> & Self :: Output { & self . arena [index] } }
    };
}

impl_16!();