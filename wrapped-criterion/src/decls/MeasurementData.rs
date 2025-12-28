macro_rules! deps {
    () => {
        Distributions!();
        Throughput!();
        Estimates!();
        ComparisonData!();
        LabeledSample!();
        Data!();
    };
}

macro_rules! MeasurementData {
    () => {
        deps!();
        pub (crate) struct MeasurementData < 'a > { pub data : Data < 'a , f64 , f64 > , pub avg_times : LabeledSample < 'a , f64 > , pub absolute_estimates : Estimates , pub distributions : Distributions , pub comparison : Option < ComparisonData > , pub throughput : Option < Throughput > , }
    };
}

MeasurementData!()