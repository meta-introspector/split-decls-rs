macro_rules! deps {
    () => {
        EarlyBinder!();
    };
}

macro_rules! GenericDefaults {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct GenericDefaults < 'db > (Option < Arc < [Option < EarlyBinder < 'db , GenericArg < 'db > > >] > >) ;
    };
}

GenericDefaults!()