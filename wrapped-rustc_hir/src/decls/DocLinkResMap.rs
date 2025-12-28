macro_rules! deps {
    () => {
        Namespace!();
        Res!();
    };
}

macro_rules! DocLinkResMap {
    () => {
        deps!();
        pub type DocLinkResMap = UnordMap < (Symbol , Namespace) , Option < Res < NodeId > > > ;
    };
}

DocLinkResMap!();