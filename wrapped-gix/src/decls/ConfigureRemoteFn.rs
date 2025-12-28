macro_rules! deps {
    () => {
        Remote!();
        Error!();
    };
}

macro_rules! ConfigureRemoteFn {
    () => {
        deps!();
        type ConfigureRemoteFn = Box < dyn FnMut (crate :: Remote < '_ >) -> Result < crate :: Remote < '_ > , Box < dyn std :: error :: Error + Send + Sync > > > ;
    };
}

ConfigureRemoteFn!()