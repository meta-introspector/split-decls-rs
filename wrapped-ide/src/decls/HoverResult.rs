macro_rules! deps {
    () => {
        HoverAction!();
        Markup!();
    };
}

macro_rules! HoverResult {
    () => {
        deps!();
        # [doc = " Contains the results when hovering over an item"] # [derive (Clone , Debug , Default , Hash , PartialEq , Eq , UpmapFromRaFixture)] pub struct HoverResult { pub markup : Markup , pub actions : Vec < HoverAction > , }
    };
}

HoverResult!();