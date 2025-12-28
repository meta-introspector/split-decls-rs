macro_rules! deps {
    () => {
        Intensity!();
        BaseColor!();
    };
}

macro_rules! Color16 {
    () => {
        deps!();
        # [doc = " A terminal color."] # [derive (Debug , PartialEq , Clone)] pub struct Color16 { base_color : BaseColor , intensity : Intensity , }
    };
}

Color16!();