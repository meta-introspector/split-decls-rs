macro_rules! deps {
    () => {
        ValidationResult!();
    };
}

macro_rules! AnalyzerExtension {
    () => {
        deps!();
        # [derive (Default)] struct AnalyzerExtension { validation_result : Mutex < Option < ValidationResult > > , }
    };
}

AnalyzerExtension!();