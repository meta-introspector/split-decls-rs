macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! Merge {
    () => {
        deps!();
        # [derive (Copy , Clone , Default)] pub struct Merge ;
    };
}

Merge!()