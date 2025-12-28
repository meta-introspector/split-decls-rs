macro_rules! deps {
    () => {
        Error!();
        Connection!();
    };
}

macro_rules! ConfigureConnectionFn {
    () => {
        deps!();
        # [cfg (any (feature = "async-network-client" , feature = "blocking-network-client"))] type ConfigureConnectionFn = Box < dyn FnMut (& mut remote :: Connection < '_ , '_ , Box < dyn Transport + Send > > ,) -> Result < () , Box < dyn std :: error :: Error + Send + Sync > > , > ;
    };
}

ConfigureConnectionFn!()