macro_rules! deps {
    () => {
        Transport!();
        Remote!();
        Error!();
    };
}

macro_rules! TransportFactory {
    () => {
        deps!();
        type TransportFactory = dyn Fn (& Remote < '_ >) -> Result < Transport , Error > + Send + Sync + 'static ;
    };
}

TransportFactory!()