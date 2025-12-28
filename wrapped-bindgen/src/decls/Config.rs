macro_rules! deps {
    () => {
        References!();
        WarningBuilder!();
        Derive!();
        TypeMap!();
    };
}

macro_rules! Config {
    () => {
        deps!();
        # [derive (Clone)] pub struct Config < 'a > { pub types : & 'a TypeMap , pub references : & 'a References , pub output : & 'a str , pub flat : bool , pub no_allow : bool , pub no_comment : bool , pub no_deps : bool , pub no_toml : bool , pub package : bool , pub rustfmt : & 'a str , pub sys : bool , pub sys_fn_ptrs : bool , pub implement : bool , pub specific_deps : bool , pub derive : & 'a Derive , pub link : & 'a str , pub warnings : & 'a WarningBuilder , pub namespace : & 'static str , }
    };
}

Config!()