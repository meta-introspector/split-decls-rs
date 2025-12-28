macro_rules! deps {
    () => {
        Error!();
        Remote!();
    };
}

macro_rules! ConfigureRemoteFn {
    () => {
        deps!();
        type ConfigureRemoteFn = Box < dyn FnMut (crate :: Remote < '_ >) -> Result < crate :: Remote < '_ > , Box < dyn std :: error :: Error + Send + Sync > > > ;
    };
}

ConfigureRemoteFn!();