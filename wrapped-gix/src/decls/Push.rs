macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! Push {
    () => {
        deps!();
        # [doc = " The `push` top-level section."] # [derive (Copy , Clone , Default)] pub struct Push ;
    };
}

Push!();