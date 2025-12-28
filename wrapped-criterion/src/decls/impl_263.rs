macro_rules! deps {
    () => {
        Sample!();
        MeasurementData!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl < 'a > MeasurementData < 'a > { pub fn iter_counts (& self) -> & Sample < f64 > { self . data . x () } # [cfg (feature = "csv_output")] pub fn sample_times (& self) -> & Sample < f64 > { self . data . y () } }
    };
}

impl_263!();