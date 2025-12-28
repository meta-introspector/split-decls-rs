macro_rules! deps {
    () => {
        Measure!();
    };
}

macro_rules! PositiveMeasure {
    () => {
        deps!();
        # [doc = " Some measure of positive numbers, assuming positive"] # [doc = " float-pointing numbers"] pub trait PositiveMeasure : Measure + Copy { fn zero () -> Self ; fn max () -> Self ; }
    };
}

PositiveMeasure!()