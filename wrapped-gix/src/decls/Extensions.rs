macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! Extensions {
    () => {
        deps!();
        # [doc = " The `extension` top-level section."] # [derive (Copy , Clone , Default)] pub struct Extensions ;
    };
}

Extensions!();