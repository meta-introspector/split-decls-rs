macro_rules! deps {
    () => {
        Plot!();
        Comparison!();
        ConfidenceInterval!();
    };
}

macro_rules! Context {
    () => {
        deps!();
        # [derive (Serialize)] struct Context { title : String , confidence : String , thumbnail_width : usize , thumbnail_height : usize , slope : Option < ConfidenceInterval > , r2 : ConfidenceInterval , mean : ConfidenceInterval , std_dev : ConfidenceInterval , median : ConfidenceInterval , mad : ConfidenceInterval , throughput : Option < ConfidenceInterval > , additional_plots : Vec < Plot > , comparison : Option < Comparison > , }
    };
}

Context!()