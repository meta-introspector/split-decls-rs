macro_rules! deps {
    () => {
        AxisScale!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl AxisScale { fn to_gnuplot (self) -> Scale { match self { AxisScale :: Linear => Scale :: Linear , AxisScale :: Logarithmic => Scale :: Logarithmic , } } }
    };
}

impl_192!()