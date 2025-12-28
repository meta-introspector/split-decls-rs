macro_rules! deps {
    () => {
        Build!();
    };
}

macro_rules! Config {
    () => {
        deps!();
        # [derive (Serialize , Debug)] pub struct Config { pub build : Build , }
    };
}

Config!()