macro_rules! deps {
    () => {
        Registry!();
    };
}

macro_rules! THE_REGISTRY {
    () => {
        deps!();
        static mut THE_REGISTRY : Option < Arc < Registry > > = None ;
    };
}

THE_REGISTRY!();