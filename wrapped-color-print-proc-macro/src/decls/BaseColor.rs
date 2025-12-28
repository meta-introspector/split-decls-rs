macro_rules! deps {
    () => {
        Intensity!();
    };
}

macro_rules! BaseColor {
    () => {
        deps!();
        # [doc = " A \"base\" terminal color, which has to be completed with an [`Intensity`] in order to describe a"] # [doc = " whole terminal color."] # [derive (Debug , PartialEq , Copy , Clone)] pub enum BaseColor { Black , Red , Green , Yellow , Blue , Magenta , Cyan , White , }
    };
}

BaseColor!();