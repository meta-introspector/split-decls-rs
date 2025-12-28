macro_rules! deps {
    () => {
        Default!();
    };
}

macro_rules! Clone {
    () => {
        deps!();
        # [doc = " The `clone` top-level section."] # [derive (Copy , Clone , Default)] pub struct Clone ;
    };
}

Clone!();