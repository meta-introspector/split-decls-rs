macro_rules! deps {
    () => {
        Res!();
        PerNS!();
        Path!();
    };
}

macro_rules! UsePath {
    () => {
        deps!();
        # [doc = " Up to three resolutions for type, value and macro namespaces."] pub type UsePath < 'hir > = Path < 'hir , PerNS < Option < Res > > > ;
    };
}

UsePath!()