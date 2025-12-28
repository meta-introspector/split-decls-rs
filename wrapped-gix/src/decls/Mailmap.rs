macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! Mailmap {
    () => {
        deps!();
        # [derive (Copy , Clone , Default)] pub struct Mailmap ;
    };
}

Mailmap!()