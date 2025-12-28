macro_rules! deps {
    () => {
        SchemaEnvInner!();
    };
}

macro_rules! SchemaEnv {
    () => {
        deps!();
        # [doc (hidden)] # [derive (Clone)] pub struct SchemaEnv (pub (crate) Arc < SchemaEnvInner >) ;
    };
}

SchemaEnv!();