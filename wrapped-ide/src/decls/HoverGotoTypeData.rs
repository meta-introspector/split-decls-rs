macro_rules! deps {
    () => {
        NavigationTarget!();
    };
}

macro_rules! HoverGotoTypeData {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq , Hash , UpmapFromRaFixture)] pub struct HoverGotoTypeData { pub mod_path : String , pub nav : NavigationTarget , }
    };
}

HoverGotoTypeData!();