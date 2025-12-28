macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! Remote {
    () => {
        deps!();
        # [doc = " The `remote` top-level section."] # [derive (Copy , Clone , Default)] pub struct Remote ;
    };
}

Remote!();