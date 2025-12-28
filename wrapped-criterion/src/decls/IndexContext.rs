macro_rules! deps {
    () => {
        BenchmarkGroup!();
    };
}

macro_rules! IndexContext {
    () => {
        deps!();
        # [derive (Serialize)] struct IndexContext < 'a > { groups : Vec < BenchmarkGroup < 'a > > , }
    };
}

IndexContext!();