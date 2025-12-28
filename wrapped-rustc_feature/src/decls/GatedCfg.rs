macro_rules! deps {
    () => {
        GateFn!();
    };
}

macro_rules! GatedCfg {
    () => {
        deps!();
        pub type GatedCfg = (Symbol , Symbol , GateFn) ;
    };
}

GatedCfg!()