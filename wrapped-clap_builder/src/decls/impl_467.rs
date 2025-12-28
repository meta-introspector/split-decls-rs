macro_rules! deps {
    () => {
        AnyValue!();
        Occurrences!();
    };
}

macro_rules! impl_467 {
    () => {
        deps!();
        impl < T > Default for Occurrences < T > { fn default () -> Self { let empty : Vec < Vec < AnyValue > > = Default :: default () ; Occurrences { iter : empty . into_iter () . map (| _ | unreachable ! ()) , } } }
    };
}

impl_467!();