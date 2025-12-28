macro_rules! deps {
    () => {
        Measure!();
    };
}

macro_rules! BoundedMeasure {
    () => {
        deps!();
        pub trait BoundedMeasure : Measure + core :: ops :: Sub < Self , Output = Self > { fn min () -> Self ; fn max () -> Self ; fn overflowing_add (self , rhs : Self) -> (Self , bool) ; fn from_f32 (val : f32) -> Self ; fn from_f64 (val : f64) -> Self ; }
    };
}

BoundedMeasure!();