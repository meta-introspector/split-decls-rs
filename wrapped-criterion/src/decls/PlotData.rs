macro_rules! deps {
    () => {
        ComparisonData!();
        ValueFormatter!();
        MeasurementData!();
    };
}

macro_rules! PlotData {
    () => {
        deps!();
        # [derive (Clone , Copy)] pub (crate) struct PlotData < 'a > { pub (crate) formatter : & 'a dyn ValueFormatter , pub (crate) measurements : & 'a MeasurementData < 'a > , pub (crate) comparison : Option < & 'a ComparisonData > , }
    };
}

PlotData!();