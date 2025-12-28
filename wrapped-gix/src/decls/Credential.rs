macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! Credential {
    () => {
        deps!();
        # [doc = " The `credential` top-level section."] # [derive (Copy , Clone , Default)] pub struct Credential ;
    };
}

Credential!();