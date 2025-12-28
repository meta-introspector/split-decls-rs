macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! Credential {
    () => {
        deps!();
        # [doc = " The `credential` top-level section."] # [derive (Copy , Clone , Default)] pub struct Credential ;
    };
}

Credential!()