macro_rules! deps {
    () => {
        AutoCfg!();
    };
}

macro_rules! new {
    () => {
        deps!();
        # [doc = " Creates a new `AutoCfg` instance."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `AutoCfg::new()` returns an error."] pub fn new () -> AutoCfg { AutoCfg :: new () . unwrap () }
    };
}

new!()