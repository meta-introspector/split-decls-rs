macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! Author {
    () => {
        deps!();
        # [doc = " The `author` top-level section."] # [derive (Copy , Clone , Default)] pub struct Author ;
    };
}

Author!();