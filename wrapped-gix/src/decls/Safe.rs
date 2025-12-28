macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! Safe {
    () => {
        deps!();
        # [doc = " The `safe` top-level section."] # [derive (Copy , Clone , Default)] pub struct Safe ;
    };
}

Safe!();