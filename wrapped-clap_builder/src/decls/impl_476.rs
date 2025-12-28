macro_rules! deps {
    () => {
        OccurrencesRef!();
        AnyValue!();
    };
}

macro_rules! impl_476 {
    () => {
        deps!();
        impl < T > Default for OccurrencesRef < '_ , T > { fn default () -> Self { static EMPTY : [Vec < AnyValue > ; 0] = [] ; OccurrencesRef { iter : EMPTY [..] . iter () . map (| _ | unreachable ! ()) , } } }
    };
}

impl_476!();