macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! Ssh {
    () => {
        deps!();
        # [doc = " The `ssh` top-level section."] # [derive (Copy , Clone , Default)] pub struct Ssh ;
    };
}

Ssh!();