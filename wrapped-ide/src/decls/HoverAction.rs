macro_rules! deps {
    () => {
        Runnable!();
        HoverGotoTypeData!();
    };
}

macro_rules! HoverAction {
    () => {
        deps!();
        # [derive (Debug , Clone , Hash , PartialEq , Eq , UpmapFromRaFixture)] pub enum HoverAction { Runnable (Runnable) , Implementation (FilePosition) , Reference (FilePosition) , GoToType (Vec < HoverGotoTypeData >) , }
    };
}

HoverAction!()