macro_rules! deps {
    () => {
        AnalysisHost!();
    };
}

macro_rules! impl_515 {
    () => {
        deps!();
        impl Default for AnalysisHost { fn default () -> AnalysisHost { AnalysisHost :: new (None) } }
    };
}

impl_515!();