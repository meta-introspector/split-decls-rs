macro_rules! deps {
    () => {
        ComparisonData!();
        PlotData!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl < 'a > PlotData < 'a > { pub fn comparison (mut self , comp : & 'a ComparisonData) -> PlotData < 'a > { self . comparison = Some (comp) ; self } }
    };
}

impl_252!();