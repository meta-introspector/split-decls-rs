macro_rules! deps {
    () => {
        AdjustMode!();
        PeelKind!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl AdjustMode { const fn peel_until_adt (opt_adt_def : Option < DefId >) -> AdjustMode { AdjustMode :: Peel { kind : PeelKind :: Implicit { until_adt : opt_adt_def , pat_ref_layers : 0 } } } const fn peel_all () -> AdjustMode { AdjustMode :: peel_until_adt (None) } }
    };
}

impl_320!();