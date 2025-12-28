macro_rules! Config {
    () => {
        # [doc = " A way to access git configuration"] pub (crate) type Config = OwnShared < gix_config :: File < 'static > > ;
    };
}

Config!()