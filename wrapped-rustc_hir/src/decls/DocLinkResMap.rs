macro_rules! deps {
    () => {
        Res!();
        Namespace!();
    };
}

macro_rules! DocLinkResMap {
    () => {
        deps!();
        pub type DocLinkResMap = UnordMap < (Symbol , Namespace) , Option < Res < NodeId > > > ;
    };
}

DocLinkResMap!()