macro_rules! deps {
    () => {
        ReportLink!();
        BenchmarkValueGroup!();
    };
}

macro_rules! BenchmarkGroup {
    () => {
        deps!();
        # [derive (Serialize)] struct BenchmarkGroup < 'a > { group_report : ReportLink < 'a > , function_ids : Option < Vec < ReportLink < 'a > > > , values : Option < Vec < ReportLink < 'a > > > , individual_links : Vec < BenchmarkValueGroup < 'a > > , }
    };
}

BenchmarkGroup!()