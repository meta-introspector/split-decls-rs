macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! Http {
    () => {
        deps!();
        # [doc = " The `http` top-level section."] # [derive (Copy , Clone , Default)] pub struct Http ;
    };
}

Http!()