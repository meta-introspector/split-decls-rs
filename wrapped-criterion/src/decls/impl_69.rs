macro_rules! deps {
    () => {
        AxisScale!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl From < crate :: AxisScale > for AxisScale { fn from (other : crate :: AxisScale) -> Self { match other { crate :: AxisScale :: Linear => AxisScale :: Linear , crate :: AxisScale :: Logarithmic => AxisScale :: Logarithmic , } } }
    };
}

impl_69!();