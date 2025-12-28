macro_rules! deps {
    () => {
        Parameter!();
    };
}

macro_rules! ParameterCollector {
    () => {
        deps!();
        struct ParameterCollector { parameters : Vec < Parameter > , include_nonconstraining : bool , }
    };
}

ParameterCollector!()