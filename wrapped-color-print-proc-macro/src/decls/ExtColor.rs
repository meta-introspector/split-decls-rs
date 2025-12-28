macro_rules! deps {
    () => {
        Color!();
    };
}

macro_rules! ExtColor {
    () => {
        deps!();
        # [doc = " An \"extended\" color, which can be either a real color or the \"normal\", default color."] # [derive (Debug , PartialEq , Clone)] pub enum ExtColor { Normal , Color (Color) , }
    };
}

ExtColor!();