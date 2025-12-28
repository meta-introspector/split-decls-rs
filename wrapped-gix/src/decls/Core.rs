macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! Core {
    () => {
        deps!();
        # [doc = " The `core` top-level section."] # [derive (Copy , Clone , Default)] pub struct Core ;
    };
}

Core!();