macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! Extensions {
    () => {
        deps!();
        # [doc = " The `extension` top-level section."] # [derive (Copy , Clone , Default)] pub struct Extensions ;
    };
}

Extensions!()