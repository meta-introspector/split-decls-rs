macro_rules! deps {
    () => {
        AnyValue!();
        Values!();
    };
}

macro_rules! impl_452 {
    () => {
        deps!();
        # [doc = " Creates an empty iterator."] impl < T > Default for Values < T > { fn default () -> Self { let empty : Vec < Vec < AnyValue > > = Default :: default () ; Values { iter : empty . into_iter () . flatten () . map (| _ | unreachable ! ()) , len : 0 , } } }
    };
}

impl_452!()