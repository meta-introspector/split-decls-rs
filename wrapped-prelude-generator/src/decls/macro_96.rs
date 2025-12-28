macro_rules! deps {
    () => {
        FunctionMetrics!();
    };
}

macro_rules! macro_96 {
    () => {
        deps!();
        lazy_static :: lazy_static ! { static ref METRICS : Mutex < HashMap < String , FunctionMetrics >> = Mutex :: new (HashMap :: new ()) ; }
    };
}

macro_96!();