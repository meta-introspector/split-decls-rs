macro_rules! deps {
    () => {
        Rgb!();
        Color!();
    };
}

macro_rules! Gradient {
    () => {
        deps!();
        # [doc = " Linear color gradient between two color stops"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Gradient { # [doc = " Start Color of Gradient"] pub start : Rgb , # [doc = " End Color of Gradient"] pub end : Rgb , }
    };
}

Gradient!();