macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! Merge {
    () => {
        deps!();
        # [derive (Copy , Clone , Default)] pub struct Merge ;
    };
}

Merge!();