macro_rules! deps {
    () => {
        AnalysisHost!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Default for AnalysisHost { fn default () -> AnalysisHost { AnalysisHost :: new (None) } }
    };
}

impl_47!()