macro_rules! deps {
    () => {
        QueryEnvInner!();
    };
}

macro_rules! QueryEnv {
    () => {
        deps!();
        # [doc (hidden)] # [derive (Clone)] pub struct QueryEnv (Arc < QueryEnvInner >) ;
    };
}

QueryEnv!();