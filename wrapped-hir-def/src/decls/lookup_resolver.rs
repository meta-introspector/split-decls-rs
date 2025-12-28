macro_rules! deps {
    () => {
        Resolver!();
        HasResolver!();
        DefDatabase!();
        AstIdLoc!();
    };
}

macro_rules! lookup_resolver {
    () => {
        deps!();
        fn lookup_resolver (db : & dyn DefDatabase , lookup : impl Lookup < Database = dyn DefDatabase , Data = impl AstIdLoc < Container = impl HasResolver > > ,) -> Resolver < '_ > { lookup . lookup (db) . container () . resolver (db) }
    };
}

lookup_resolver!()