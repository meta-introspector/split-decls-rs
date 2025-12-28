macro_rules! Intensity {
    () => {
        # [doc = " The intensity of a terminal color."] # [derive (Debug , PartialEq , Copy , Clone)] pub enum Intensity { Normal , Bright , }
    };
}

Intensity!();