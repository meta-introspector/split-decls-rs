macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! Pack {
    () => {
        deps!();
        # [doc = " The `pack` top-level section."] # [derive (Copy , Clone , Default)] pub struct Pack ;
    };
}

Pack!();