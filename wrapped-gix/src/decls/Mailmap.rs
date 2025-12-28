macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! Mailmap {
    () => {
        deps!();
        # [derive (Copy , Clone , Default)] pub struct Mailmap ;
    };
}

Mailmap!();