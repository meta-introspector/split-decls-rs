macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! Init {
    () => {
        deps!();
        # [doc = " The `init` top-level section."] # [derive (Copy , Clone , Default)] pub struct Init ;
    };
}

Init!();