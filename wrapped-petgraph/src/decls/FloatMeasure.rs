macro_rules! deps {
    () => {
        Measure!();
    };
}

macro_rules! FloatMeasure {
    () => {
        deps!();
        # [doc = " A floating-point measure."] pub trait FloatMeasure : Measure + Copy { fn zero () -> Self ; fn infinite () -> Self ; fn from_f32 (val : f32) -> Self ; fn from_f64 (val : f64) -> Self ; }
    };
}

FloatMeasure!();