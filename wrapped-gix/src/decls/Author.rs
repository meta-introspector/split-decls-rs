macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! Author {
    () => {
        deps!();
        # [doc = " The `author` top-level section."] # [derive (Copy , Clone , Default)] pub struct Author ;
    };
}

Author!()