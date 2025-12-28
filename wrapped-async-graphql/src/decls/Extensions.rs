macro_rules! deps {
    () => {
        SchemaEnv!();
        Data!();
        Extension!();
    };
}

macro_rules! Extensions {
    () => {
        deps!();
        # [derive (Clone)] # [doc (hidden)] pub struct Extensions { extensions : Vec < Arc < dyn Extension > > , schema_env : SchemaEnv , session_data : Arc < Data > , query_data : Option < Arc < Data > > , }
    };
}

Extensions!()