macro_rules! deps {
    () => {
        ReportLink!();
    };
}

macro_rules! BenchmarkValueGroup {
    () => {
        deps!();
        # [derive (Serialize)] struct BenchmarkValueGroup < 'a > { value : Option < ReportLink < 'a > > , benchmarks : Vec < ReportLink < 'a > > , }
    };
}

BenchmarkValueGroup!();