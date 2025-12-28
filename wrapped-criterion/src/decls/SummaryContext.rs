macro_rules! deps {
    () => {
        IndividualBenchmark!();
    };
}

macro_rules! SummaryContext {
    () => {
        deps!();
        # [derive (Serialize)] struct SummaryContext { group_id : String , thumbnail_width : usize , thumbnail_height : usize , violin_plot : Option < String > , line_chart : Option < String > , benchmarks : Vec < IndividualBenchmark > , }
    };
}

SummaryContext!();