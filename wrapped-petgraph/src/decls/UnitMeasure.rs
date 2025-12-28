macro_rules! deps {
    () => {
        Measure!();
    };
}

macro_rules! UnitMeasure {
    () => {
        deps!();
        # [doc = " A floating-point measure that can be computed from `usize`"] # [doc = " and with a default measure of proximity.  "] pub trait UnitMeasure : Measure + core :: ops :: Sub < Self , Output = Self > + core :: ops :: Mul < Self , Output = Self > + core :: ops :: Div < Self , Output = Self > + core :: iter :: Sum { fn zero () -> Self ; fn one () -> Self ; fn from_usize (nb : usize) -> Self ; fn default_tol () -> Self ; fn from_f32 (val : f32) -> Self ; fn from_f64 (val : f64) -> Self ; }
    };
}

UnitMeasure!();