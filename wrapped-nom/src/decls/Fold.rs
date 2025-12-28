macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Fold {
    () => {
        deps!();
        # [doc = " Parser implementation for the [fold] combinator"] pub struct Fold < F , G , H , Range > { parser : F , init : H , fold : G , range : Range , }
    };
}

Fold!();