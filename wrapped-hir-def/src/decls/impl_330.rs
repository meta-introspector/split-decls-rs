macro_rules! deps {
    () => {
        AdtId!();
        Resolver!();
        HasResolver!();
        DefDatabase!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl < T : Into < AdtId > + Copy > HasResolver for T { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { let def = self . into () ; def . module (db) . resolver (db) . push_generic_params_scope (db , def . into ()) } }
    };
}

impl_330!()