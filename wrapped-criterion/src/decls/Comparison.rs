macro_rules! deps {
    () => {
        ConfidenceInterval!();
        Plot!();
    };
}

macro_rules! Comparison {
    () => {
        deps!();
        # [derive (Serialize)] struct Comparison { p_value : String , inequality : String , significance_level : String , explanation : String , change : ConfidenceInterval , thrpt_change : Option < ConfidenceInterval > , additional_plots : Vec < Plot > , }
    };
}

Comparison!()