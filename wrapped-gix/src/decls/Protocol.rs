macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! Protocol {
    () => {
        deps!();
        # [doc = " The `protocol` top-level section."] # [derive (Copy , Clone , Default)] pub struct Protocol ;
    };
}

Protocol!();