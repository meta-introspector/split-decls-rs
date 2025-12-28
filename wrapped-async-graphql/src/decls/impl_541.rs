macro_rules! deps {
    () => {
        Inner!();
        Extension!();
        ExtensionFactory!();
        ApolloTracing!();
        ApolloTracingExtension!();
    };
}

macro_rules! impl_541 {
    () => {
        deps!();
        impl ExtensionFactory for ApolloTracing { fn create (& self) -> Arc < dyn Extension > { Arc :: new (ApolloTracingExtension { inner : Mutex :: new (Inner { start_time : Utc :: now () , end_time : Utc :: now () , resolves : Default :: default () , }) , }) } }
    };
}

impl_541!();