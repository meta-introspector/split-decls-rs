macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! Pack {
    () => {
        deps!();
        # [doc = " The `pack` top-level section."] # [derive (Copy , Clone , Default)] pub struct Pack ;
    };
}

Pack!()